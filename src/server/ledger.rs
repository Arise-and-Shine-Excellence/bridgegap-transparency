//! The double-entry ledger. Every change to money or stock goes through here.
//!
//! A transfer moves `amount` of ONE asset from the credit account (gives) to
//! the debit account (receives). Transfers are append-only; `reverse_*` adds
//! mirror-image transfers instead of deleting anything.
//!
//! Flows (see docs/architecture.md):
//! - Production batch, for each raw material used:
//!   ASE raw_materials  -> group raw_materials   (the group takes it from ASE's store)
//!   group raw_materials -> group cost           (and uses it up in the batch)
//!   and for the money:  group cash -> ASE cash    (material cost at fixed prices)
//!   and the product:    Production external -> group stock
//! - Sale:
//!   group stock -> Customers external           (product out)
//!   Customers external -> group cash            (money in, at the fixed price)

use crate::error::{AppError, Field};
use crate::model::Price;
use sqlx::{Sqlite, SqliteConnection, SqlitePool, Transaction};
use std::collections::HashMap;

pub const ORG_ASE: i64 = 1;
pub const ORG_CUSTOMERS: i64 = 2;
pub const ORG_PRODUCTION: i64 = 3;
pub const ASSET_TZS: i64 = 1;

type Tx = Transaction<'static, Sqlite>;

/// Find or create the account for (organisation, asset, kind).
pub async fn account(
    conn: &mut SqliteConnection,
    org: i64,
    asset: i64,
    kind: &str,
) -> Result<i64, sqlx::Error> {
    sqlx::query("INSERT INTO account (organisation_id, asset_id, kind) VALUES (?, ?, ?) ON CONFLICT DO NOTHING")
        .bind(org)
        .bind(asset)
        .bind(kind)
        .execute(&mut *conn)
        .await?;
    sqlx::query_scalar(
        "SELECT id FROM account WHERE organisation_id = ? AND asset_id = ? AND kind = ?",
    )
    .bind(org)
    .bind(asset)
    .bind(kind)
    .fetch_one(&mut *conn)
    .await
}

/// Balance of (organisation, asset, kind): debits minus credits.
pub async fn balance(
    conn: &mut SqliteConnection,
    org: i64,
    asset: i64,
    kind: &str,
) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar(
        "SELECT COALESCE(SUM(CASE WHEN t.debit_account_id = a.id THEN t.amount ELSE -t.amount END), 0)
         FROM account a
         JOIN transfer t ON t.debit_account_id = a.id OR t.credit_account_id = a.id
         WHERE a.organisation_id = ? AND a.asset_id = ? AND a.kind = ?",
    )
    .bind(org)
    .bind(asset)
    .bind(kind)
    .fetch_one(&mut *conn)
    .await
}

struct NewTransfer<'a> {
    key: String,
    debit: i64,
    credit: i64,
    amount: i64,
    by: i64,
    batch_id: Option<i64>,
    sale_id: Option<i64>,
    reverses_id: Option<i64>,
    at: &'a str,
}

async fn post(tx: &mut Tx, t: NewTransfer<'_>) -> Result<(), sqlx::Error> {
    if t.amount == 0 {
        return Ok(());
    }
    sqlx::query(
        "INSERT INTO transfer (idempotency_key, debit_account_id, credit_account_id, amount,
                               created_at, created_by, batch_id, sale_id, reverses_id)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(t.key)
    .bind(t.debit)
    .bind(t.credit)
    .bind(t.amount)
    .bind(t.at)
    .bind(t.by)
    .bind(t.batch_id)
    .bind(t.sale_id)
    .bind(t.reverses_id)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

/// Current fixed prices of one kind ('sale' or 'material'), by asset id.
pub async fn current_prices(
    conn: &mut SqliteConnection,
    kind: &str,
) -> Result<HashMap<i64, Price>, sqlx::Error> {
    let rows: Vec<(i64, i64, i64)> = sqlx::query_as(
        "SELECT p.asset_id, p.amount_tzs, p.per_quantity FROM price p
         WHERE p.kind = ?1 AND p.id = (
             SELECT p2.id FROM price p2
             WHERE p2.asset_id = p.asset_id AND p2.kind = ?1 AND p2.valid_from <= ?2
             ORDER BY p2.valid_from DESC, p2.id DESC LIMIT 1)",
    )
    .bind(kind)
    .bind(super::now_utc())
    .fetch_all(&mut *conn)
    .await?;
    Ok(rows
        .into_iter()
        .map(|(id, amount_tzs, per_quantity)| {
            (
                id,
                Price {
                    amount_tzs,
                    per_quantity,
                },
            )
        })
        .collect())
}

async fn asset_kind(
    conn: &mut SqliteConnection,
    asset: i64,
) -> Result<Option<String>, sqlx::Error> {
    sqlx::query_scalar("SELECT kind FROM asset WHERE id = ?")
        .bind(asset)
        .fetch_optional(&mut *conn)
        .await
}

fn is_unique_violation(e: &sqlx::Error) -> bool {
    matches!(e, sqlx::Error::Database(d) if d.is_unique_violation())
}

pub struct BatchInput {
    pub key: String,
    pub organisation_id: i64,
    pub product_id: i64,
    pub quantity: i64,
    pub produced_on: String,
    pub site: String,
    pub comment: String,
    /// (material asset id, base units used)
    pub ingredients: Vec<(i64, i64)>,
    pub by: i64,
}

/// Record a production batch. Safe to call twice with the same key: the second
/// call returns the first batch's id and records nothing.
pub async fn record_batch(pool: &SqlitePool, input: BatchInput) -> Result<i64, AppError> {
    if let Some(id) = existing(pool, "batch", &input.key).await? {
        return Ok(id);
    }
    if input.quantity <= 0 {
        return Err(AppError::Invalid(Field::Quantity));
    }
    let mut tx = pool.begin().await?;
    if asset_kind(&mut tx, input.product_id).await?.as_deref() != Some("product") {
        return Err(AppError::Invalid(Field::Product));
    }
    let prices = current_prices(&mut tx, "material").await?;
    let at = super::now_utc();

    let inserted = sqlx::query(
        "INSERT INTO batch (idempotency_key, organisation_id, product_asset_id, quantity, site,
                            produced_on, comment, created_at, created_by)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&input.key)
    .bind(input.organisation_id)
    .bind(input.product_id)
    .bind(input.quantity)
    .bind(&input.site)
    .bind(&input.produced_on)
    .bind(&input.comment)
    .bind(&at)
    .bind(input.by)
    .execute(&mut *tx)
    .await;
    let batch_id = match inserted {
        Ok(r) => r.last_insert_rowid(),
        Err(e) if is_unique_violation(&e) => {
            drop(tx);
            return existing(pool, "batch", &input.key)
                .await?
                .ok_or(AppError::Internal);
        }
        Err(e) => return Err(e.into()),
    };

    let org = input.organisation_id;
    let mut total_cost = 0;
    for (asset, amount) in input.ingredients.iter().copied().filter(|(_, a)| *a > 0) {
        if asset_kind(&mut tx, asset).await?.as_deref() != Some("material") {
            return Err(AppError::Invalid(Field::Product));
        }
        sqlx::query("INSERT INTO batch_ingredient (batch_id, asset_id, amount) VALUES (?, ?, ?)")
            .bind(batch_id)
            .bind(asset)
            .bind(amount)
            .execute(&mut *tx)
            .await?;
        let ase_raw = account(&mut tx, ORG_ASE, asset, "raw_materials").await?;
        let group_raw = account(&mut tx, org, asset, "raw_materials").await?;
        let group_cost = account(&mut tx, org, asset, "cost").await?;
        let common = |suffix: &str| format!("{}:{suffix}:{asset}", input.key);
        post(
            &mut tx,
            NewTransfer {
                key: common("take"),
                debit: group_raw,
                credit: ase_raw,
                amount,
                by: input.by,
                batch_id: Some(batch_id),
                sale_id: None,
                reverses_id: None,
                at: &at,
            },
        )
        .await?;
        post(
            &mut tx,
            NewTransfer {
                key: common("use"),
                debit: group_cost,
                credit: group_raw,
                amount,
                by: input.by,
                batch_id: Some(batch_id),
                sale_id: None,
                reverses_id: None,
                at: &at,
            },
        )
        .await?;
        total_cost += prices.get(&asset).map(|p| p.value_of(amount)).unwrap_or(0);
    }

    let group_cash = account(&mut tx, org, ASSET_TZS, "cash").await?;
    let ase_cash = account(&mut tx, ORG_ASE, ASSET_TZS, "cash").await?;
    post(
        &mut tx,
        NewTransfer {
            key: format!("{}:pay", input.key),
            debit: ase_cash,
            credit: group_cash,
            amount: total_cost,
            by: input.by,
            batch_id: Some(batch_id),
            sale_id: None,
            reverses_id: None,
            at: &at,
        },
    )
    .await?;

    let produced = account(&mut tx, ORG_PRODUCTION, input.product_id, "external").await?;
    let group_stock = account(&mut tx, org, input.product_id, "stock").await?;
    post(
        &mut tx,
        NewTransfer {
            key: format!("{}:make", input.key),
            debit: group_stock,
            credit: produced,
            amount: input.quantity,
            by: input.by,
            batch_id: Some(batch_id),
            sale_id: None,
            reverses_id: None,
            at: &at,
        },
    )
    .await?;

    tx.commit().await?;
    Ok(batch_id)
}

pub struct SaleInput {
    pub key: String,
    pub organisation_id: i64,
    pub product_id: i64,
    pub quantity: i64,
    pub sold_on: String,
    pub comment: String,
    pub by: i64,
}

/// Record a sale at the fixed price. Refuses to sell more than is in stock.
/// Idempotent like `record_batch`. Returns (sale id, amount in TZS).
pub async fn record_sale(pool: &SqlitePool, input: SaleInput) -> Result<(i64, i64), AppError> {
    if let Some(id) = existing(pool, "sale", &input.key).await? {
        let amount: i64 = sqlx::query_scalar("SELECT amount_tzs FROM sale_view WHERE id = ?")
            .bind(id)
            .fetch_one(pool)
            .await?;
        return Ok((id, amount));
    }
    if input.quantity <= 0 {
        return Err(AppError::Invalid(Field::Quantity));
    }
    let mut tx = pool.begin().await?;
    if asset_kind(&mut tx, input.product_id).await?.as_deref() != Some("product") {
        return Err(AppError::Invalid(Field::Product));
    }
    let price = *current_prices(&mut tx, "sale")
        .await?
        .get(&input.product_id)
        .ok_or(AppError::NoPrice)?;
    let org = input.organisation_id;
    let in_stock = balance(&mut tx, org, input.product_id, "stock").await?;
    if in_stock < input.quantity {
        return Err(AppError::InsufficientStock {
            available: format_stock(&mut tx, input.product_id, in_stock).await?,
        });
    }
    let at = super::now_utc();
    let inserted = sqlx::query(
        "INSERT INTO sale (idempotency_key, organisation_id, member_id, product_asset_id, quantity, sold_on, comment, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&input.key)
    .bind(org)
    .bind(input.by)
    .bind(input.product_id)
    .bind(input.quantity)
    .bind(&input.sold_on)
    .bind(&input.comment)
    .bind(&at)
    .execute(&mut *tx)
    .await;
    let sale_id = match inserted {
        Ok(r) => r.last_insert_rowid(),
        Err(e) if is_unique_violation(&e) => {
            drop(tx);
            let id = existing(pool, "sale", &input.key)
                .await?
                .ok_or(AppError::Internal)?;
            let amount: i64 = sqlx::query_scalar("SELECT amount_tzs FROM sale_view WHERE id = ?")
                .bind(id)
                .fetch_one(pool)
                .await?;
            return Ok((id, amount));
        }
        Err(e) => return Err(e.into()),
    };

    let amount = price.value_of(input.quantity);
    let group_stock = account(&mut tx, org, input.product_id, "stock").await?;
    let customers_stock = account(&mut tx, ORG_CUSTOMERS, input.product_id, "external").await?;
    let group_cash = account(&mut tx, org, ASSET_TZS, "cash").await?;
    let customers_cash = account(&mut tx, ORG_CUSTOMERS, ASSET_TZS, "external").await?;
    post(
        &mut tx,
        NewTransfer {
            key: format!("{}:out", input.key),
            debit: customers_stock,
            credit: group_stock,
            amount: input.quantity,
            by: input.by,
            batch_id: None,
            sale_id: Some(sale_id),
            reverses_id: None,
            at: &at,
        },
    )
    .await?;
    post(
        &mut tx,
        NewTransfer {
            key: format!("{}:pay", input.key),
            debit: group_cash,
            credit: customers_cash,
            amount,
            by: input.by,
            batch_id: None,
            sale_id: Some(sale_id),
            reverses_id: None,
            at: &at,
        },
    )
    .await?;
    tx.commit().await?;
    Ok((sale_id, amount))
}

async fn format_stock(
    conn: &mut SqliteConnection,
    asset: i64,
    qty: i64,
) -> Result<String, sqlx::Error> {
    let lang = crate::i18n::request_lang();
    let a = super::queries::asset(conn, asset).await?;
    Ok(a.fmt_qty_unit(qty, lang))
}

async fn existing(pool: &SqlitePool, table: &str, key: &str) -> Result<Option<i64>, sqlx::Error> {
    let sql = match table {
        "batch" => "SELECT id FROM batch WHERE idempotency_key = ?",
        _ => "SELECT id FROM sale WHERE idempotency_key = ?",
    };
    sqlx::query_scalar(sql).bind(key).fetch_optional(pool).await
}

pub enum Record {
    Batch(i64),
    Sale(i64),
}

/// Undo a batch or sale by posting the mirror image of each of its transfers.
/// Undoing twice is a no-op. A batch cannot be undone once its product has
/// been sold (the group's stock would go below zero).
pub async fn reverse(
    pool: &SqlitePool,
    record: Record,
    organisation_id: i64,
    by: i64,
) -> Result<(), AppError> {
    let mut tx = pool.begin().await?;
    let (id, is_batch, record_sql, transfers_sql) = match record {
        Record::Batch(id) => (
            id,
            true,
            "SELECT organisation_id, product_asset_id, quantity FROM batch WHERE id = ?",
            "SELECT id, idempotency_key, debit_account_id, credit_account_id, amount FROM transfer WHERE batch_id = ? ORDER BY id",
        ),
        Record::Sale(id) => (
            id,
            false,
            "SELECT organisation_id, product_asset_id, quantity FROM sale WHERE id = ?",
            "SELECT id, idempotency_key, debit_account_id, credit_account_id, amount FROM transfer WHERE sale_id = ? ORDER BY id",
        ),
    };
    let (owner_org, product, quantity): (i64, i64, i64) = sqlx::query_as(record_sql)
        .bind(id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or(AppError::Invalid(Field::Product))?;
    if owner_org != organisation_id {
        return Err(AppError::Forbidden);
    }
    let originals: Vec<(i64, String, i64, i64, i64)> = sqlx::query_as(transfers_sql)
        .bind(id)
        .fetch_all(&mut *tx)
        .await?;
    if originals.is_empty() || originals.iter().any(|(_, key, ..)| key.starts_with("rev:")) {
        return Ok(()); // nothing to undo, or already undone
    }
    if is_batch && balance(&mut tx, owner_org, product, "stock").await? < quantity {
        return Err(AppError::AlreadySold);
    }
    let at = super::now_utc();
    let (batch_id, sale_id) = if is_batch {
        (Some(id), None)
    } else {
        (None, Some(id))
    };
    for (tid, key, debit, credit, amount) in originals {
        post(
            &mut tx,
            NewTransfer {
                key: format!("rev:{key}"),
                debit: credit,
                credit: debit,
                amount,
                by,
                batch_id,
                sale_id,
                reverses_id: Some(tid),
                at: &at,
            },
        )
        .await?;
    }
    tx.commit().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Role;
    use crate::server::{auth, db::test_pool};

    const LIQUID: i64 = 2;
    const SULPHONIC: i64 = 10;
    const CAUSTIC: i64 = 12;

    async fn setup() -> (SqlitePool, i64, i64) {
        let pool = test_pool().await;
        let group: i64 =
            sqlx::query("INSERT INTO organisation (name, kind) VALUES ('Amka Kumekucha', 'group')")
                .execute(&pool)
                .await
                .unwrap()
                .last_insert_rowid();
        let member =
            auth::create_member(&pool, group, "Leader", "0700000001", "1234", Role::Leader)
                .await
                .unwrap();
        (pool, group, member)
    }

    fn batch(key: &str, group: i64, by: i64) -> BatchInput {
        BatchInput {
            key: key.into(),
            organisation_id: group,
            product_id: LIQUID,
            quantity: 20_000, // 20 L
            produced_on: "2026-11-05".into(),
            site: "Dongobesh".into(),
            comment: String::new(),
            ingredients: vec![(SULPHONIC, 1000), (CAUSTIC, 200)],
            by,
        }
    }

    fn sale(key: &str, group: i64, by: i64, qty: i64) -> SaleInput {
        SaleInput {
            key: key.into(),
            organisation_id: group,
            product_id: LIQUID,
            quantity: qty,
            sold_on: "2026-11-05".into(),
            comment: String::new(),
            by,
        }
    }

    async fn total_per_asset(pool: &SqlitePool) -> Vec<(i64, i64)> {
        // For every asset, sum of all balances must be zero: nothing is created or lost.
        sqlx::query_as(
            "SELECT a.asset_id, SUM(CASE WHEN t.debit_account_id = a.id THEN t.amount ELSE -t.amount END)
             FROM account a JOIN transfer t ON t.debit_account_id = a.id OR t.credit_account_id = a.id
             GROUP BY a.asset_id",
        )
        .fetch_all(pool)
        .await
        .unwrap()
    }

    #[tokio::test]
    async fn batch_moves_materials_money_and_product() {
        let (pool, group, by) = setup().await;
        record_batch(&pool, batch("k1", group, by)).await.unwrap();
        let mut c = pool.acquire().await.unwrap();
        assert_eq!(
            balance(&mut c, group, LIQUID, "stock").await.unwrap(),
            20_000
        );
        // 1 L sulphonic at 8,000/L + 0.2 kg caustic at 4,000/kg
        assert_eq!(
            balance(&mut c, group, ASSET_TZS, "cash").await.unwrap(),
            -8_800
        );
        assert_eq!(
            balance(&mut c, ORG_ASE, ASSET_TZS, "cash").await.unwrap(),
            8_800
        );
        assert_eq!(balance(&mut c, group, CAUSTIC, "cost").await.unwrap(), 200);
        assert_eq!(
            balance(&mut c, group, CAUSTIC, "raw_materials")
                .await
                .unwrap(),
            0
        );
        drop(c);
        assert!(total_per_asset(&pool)
            .await
            .iter()
            .all(|(_, sum)| *sum == 0));
    }

    #[tokio::test]
    async fn same_key_records_once() {
        let (pool, group, by) = setup().await;
        let a = record_batch(&pool, batch("same", group, by)).await.unwrap();
        let b = record_batch(&pool, batch("same", group, by)).await.unwrap();
        assert_eq!(a, b);
        let (s1, amount) = record_sale(&pool, sale("s", group, by, 1500))
            .await
            .unwrap();
        let (s2, _) = record_sale(&pool, sale("s", group, by, 1500))
            .await
            .unwrap();
        assert_eq!(s1, s2);
        assert_eq!(amount, 3_000);
        let mut c = pool.acquire().await.unwrap();
        assert_eq!(
            balance(&mut c, group, LIQUID, "stock").await.unwrap(),
            18_500
        );
    }

    #[tokio::test]
    async fn cannot_sell_more_than_stock() {
        let (pool, group, by) = setup().await;
        record_batch(&pool, batch("k", group, by)).await.unwrap();
        let err = record_sale(&pool, sale("s", group, by, 25_000))
            .await
            .unwrap_err();
        assert!(matches!(err, AppError::InsufficientStock { .. }));
    }

    #[tokio::test]
    async fn undo_restores_balances_and_keeps_history() {
        let (pool, group, by) = setup().await;
        let b = record_batch(&pool, batch("k", group, by)).await.unwrap();
        let (s, _) = record_sale(&pool, sale("s", group, by, 20_000))
            .await
            .unwrap();
        // Everything is sold, so the batch cannot be undone yet.
        assert_eq!(
            reverse(&pool, Record::Batch(b), group, by).await,
            Err(AppError::AlreadySold)
        );
        reverse(&pool, Record::Sale(s), group, by).await.unwrap();
        reverse(&pool, Record::Sale(s), group, by).await.unwrap(); // no-op
        reverse(&pool, Record::Batch(b), group, by).await.unwrap();
        let mut c = pool.acquire().await.unwrap();
        assert_eq!(balance(&mut c, group, LIQUID, "stock").await.unwrap(), 0);
        assert_eq!(balance(&mut c, group, ASSET_TZS, "cash").await.unwrap(), 0);
        drop(c);
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM transfer")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count, 2 * (2 * 2 + 2 + 2)); // originals + reversals
                                                // Other groups cannot undo it.
        assert_eq!(
            reverse(&pool, Record::Sale(s), 999, by).await,
            Err(AppError::Forbidden)
        );
    }

    #[tokio::test]
    async fn transfers_are_append_only() {
        let (pool, group, by) = setup().await;
        record_batch(&pool, batch("k", group, by)).await.unwrap();
        assert!(sqlx::query("UPDATE transfer SET amount = 1")
            .execute(&pool)
            .await
            .is_err());
        assert!(sqlx::query("DELETE FROM transfer")
            .execute(&pool)
            .await
            .is_err());
    }
}
