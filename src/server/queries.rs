//! Read models: everything the pages show, derived from the ledger.

use super::ledger;
use crate::model::*;
use chrono::Datelike;
use sqlx::SqliteConnection;

pub async fn asset(conn: &mut SqliteConnection, id: i64) -> Result<Asset, sqlx::Error> {
    sqlx::query_as(
        "SELECT id, code, kind, factor, unit_en, unit_sw, name_en, name_sw FROM asset WHERE id = ?",
    )
    .bind(id)
    .fetch_one(&mut *conn)
    .await
}

pub async fn assets(conn: &mut SqliteConnection, kind: &str) -> Result<Vec<Asset>, sqlx::Error> {
    sqlx::query_as("SELECT id, code, kind, factor, unit_en, unit_sw, name_en, name_sw FROM asset WHERE kind = ? ORDER BY sort, id")
        .bind(kind)
        .fetch_all(&mut *conn)
        .await
}

/// Assets of `asset_kind` with their current price of `price_kind`.
pub async fn priced(
    conn: &mut SqliteConnection,
    asset_kind: &str,
    price_kind: &str,
) -> Result<Vec<PricedAsset>, sqlx::Error> {
    let prices = ledger::current_prices(conn, price_kind).await?;
    Ok(assets(conn, asset_kind)
        .await?
        .into_iter()
        .map(|asset| PricedAsset {
            price: prices.get(&asset.id).copied(),
            asset,
        })
        .collect())
}

pub async fn group(conn: &mut SqliteConnection, id: i64) -> Result<Option<GroupRef>, sqlx::Error> {
    let row: Option<(i64, String)> =
        sqlx::query_as("SELECT id, name FROM organisation WHERE id = ? AND kind = 'group'")
            .bind(id)
            .fetch_optional(&mut *conn)
            .await?;
    Ok(row.map(|(id, name)| GroupRef { id, name }))
}

pub async fn groups(conn: &mut SqliteConnection) -> Result<Vec<GroupRef>, sqlx::Error> {
    let rows: Vec<(i64, String)> =
        sqlx::query_as("SELECT id, name FROM organisation WHERE kind = 'group' ORDER BY name")
            .fetch_all(&mut *conn)
            .await?;
    Ok(rows
        .into_iter()
        .map(|(id, name)| GroupRef { id, name })
        .collect())
}

/// (product id, base units in stock) for every product.
pub async fn stock(conn: &mut SqliteConnection, org: i64) -> Result<Vec<(i64, i64)>, sqlx::Error> {
    let mut out = vec![];
    for a in assets(conn, "product").await? {
        out.push((a.id, ledger::balance(conn, org, a.id, "stock").await?));
    }
    Ok(out)
}

pub async fn last_site(conn: &mut SqliteConnection, org: i64) -> Result<String, sqlx::Error> {
    let site: Option<String> = sqlx::query_scalar(
        "SELECT site FROM batch WHERE organisation_id = ? ORDER BY id DESC LIMIT 1",
    )
    .bind(org)
    .fetch_optional(&mut *conn)
    .await?;
    Ok(site.unwrap_or_default())
}

#[allow(clippy::type_complexity)]
pub async fn batches(
    conn: &mut SqliteConnection,
    org: i64,
    limit: i64,
) -> Result<Vec<BatchRow>, sqlx::Error> {
    let rows: Vec<(i64, i64, i64, String, String, String, i64, bool, String)> = sqlx::query_as(
        "SELECT b.id, b.product_asset_id, b.quantity, b.produced_on, b.site, b.comment, b.material_cost, b.reversed, m.name
         FROM batch_view b JOIN member m ON m.id = b.created_by
         WHERE b.organisation_id = ? ORDER BY b.produced_on DESC, b.id DESC LIMIT ?",
    )
    .bind(org)
    .bind(limit)
    .fetch_all(&mut *conn)
    .await?;
    let mut out = Vec::with_capacity(rows.len());
    for (id, product, quantity, produced_on, site, comment, material_cost, reversed, recorded_by) in
        rows
    {
        let ing: Vec<(i64, i64)> = sqlx::query_as(
            "SELECT asset_id, amount FROM batch_ingredient WHERE batch_id = ? ORDER BY asset_id",
        )
        .bind(id)
        .fetch_all(&mut *conn)
        .await?;
        let mut ingredients = vec![];
        for (a, amount) in ing {
            ingredients.push(Ingredient {
                asset: asset(conn, a).await?,
                amount,
            });
        }
        out.push(BatchRow {
            id,
            product: asset(conn, product).await?,
            quantity,
            produced_on,
            site,
            comment,
            material_cost,
            reversed,
            recorded_by,
            ingredients,
        });
    }
    Ok(out)
}

#[allow(clippy::type_complexity)]
pub async fn sales(
    conn: &mut SqliteConnection,
    org: i64,
    limit: i64,
) -> Result<Vec<SaleRow>, sqlx::Error> {
    let rows: Vec<(i64, i64, i64, String, String, i64, bool, String)> = sqlx::query_as(
        "SELECT s.id, s.product_asset_id, s.quantity, s.sold_on, s.comment, s.amount_tzs, s.reversed, m.name
         FROM sale_view s JOIN member m ON m.id = s.member_id
         WHERE s.organisation_id = ? ORDER BY s.sold_on DESC, s.id DESC LIMIT ?",
    )
    .bind(org)
    .bind(limit)
    .fetch_all(&mut *conn)
    .await?;
    let mut out = Vec::with_capacity(rows.len());
    for (id, product, quantity, sold_on, comment, amount_tzs, reversed, recorded_by) in rows {
        out.push(SaleRow {
            id,
            product: asset(conn, product).await?,
            quantity,
            sold_on,
            comment,
            amount_tzs,
            reversed,
            recorded_by,
        });
    }
    Ok(out)
}

fn month_prefix(year: i32, month: u32) -> String {
    format!("{year:04}-{month:02}-%")
}

async fn month_totals(
    conn: &mut SqliteConnection,
    org: i64,
    prefix: &str,
) -> Result<(i64, i64), sqlx::Error> {
    let revenue: i64 = sqlx::query_scalar("SELECT COALESCE(SUM(amount_tzs), 0) FROM sale_view WHERE organisation_id = ? AND NOT reversed AND sold_on LIKE ?")
        .bind(org)
        .bind(prefix)
        .fetch_one(&mut *conn)
        .await?;
    let cost: i64 = sqlx::query_scalar("SELECT COALESCE(SUM(material_cost), 0) FROM batch_view WHERE organisation_id = ? AND NOT reversed AND produced_on LIKE ?")
        .bind(org)
        .bind(prefix)
        .fetch_one(&mut *conn)
        .await?;
    Ok((revenue, cost))
}

pub async fn dashboard(
    conn: &mut SqliteConnection,
    group: GroupRef,
) -> Result<Dashboard, sqlx::Error> {
    let today = super::today_eat();
    let (year, month) = (today.year(), today.month());
    let prefix = month_prefix(year, month);
    let org = group.id;
    let (revenue, material_cost) = month_totals(conn, org, &prefix).await?;

    let mut products = vec![];
    let mut alerts = vec![];
    let mut unsold_value = 0;
    for product in priced(conn, "product", "sale").await? {
        let id = product.asset.id;
        let in_stock = ledger::balance(conn, org, id, "stock").await?;
        let sold_month: i64 = sqlx::query_scalar("SELECT COALESCE(SUM(quantity), 0) FROM sale_view WHERE organisation_id = ? AND product_asset_id = ? AND NOT reversed AND sold_on LIKE ?")
            .bind(org).bind(id).bind(&prefix).fetch_one(&mut *conn).await?;
        let produced_month: i64 = sqlx::query_scalar("SELECT COALESCE(SUM(quantity), 0) FROM batch_view WHERE organisation_id = ? AND product_asset_id = ? AND NOT reversed AND produced_on LIKE ?")
            .bind(org).bind(id).bind(&prefix).fetch_one(&mut *conn).await?;
        let last: Option<(i64, i64)> = sqlx::query_as("SELECT material_cost, quantity FROM batch_view WHERE organisation_id = ? AND product_asset_id = ? AND NOT reversed ORDER BY produced_on DESC, id DESC LIMIT 1")
            .bind(org).bind(id).fetch_optional(&mut *conn).await?;
        let last_unit_cost = last.filter(|(cost, _)| *cost > 0).map(|(cost, qty)| {
            Price {
                amount_tzs: cost,
                per_quantity: qty,
            }
            .value_of(product.asset.factor)
        });

        if let (Some(unit_cost), Some(price)) = (last_unit_cost, product.price_per_unit()) {
            if unit_cost >= price {
                alerts.push(Alert::UnitLoss {
                    product: product.asset.clone(),
                    unit_cost,
                    price,
                });
            }
        }
        if in_stock < 0 {
            alerts.push(Alert::NegativeStock {
                product: product.asset.clone(),
            });
        }
        if let Some(p) = product.price {
            unsold_value += p.value_of(in_stock.max(0));
        }
        products.push(ProductLine {
            product,
            in_stock,
            sold_month,
            produced_month,
            last_unit_cost,
        });
    }
    if revenue < material_cost {
        alerts.push(Alert::MonthLoss {
            loss: material_cost - revenue,
            unsold_value,
        });
    }
    Ok(Dashboard {
        group,
        year,
        month,
        revenue,
        material_cost,
        products,
        alerts,
    })
}

pub async fn group_summaries(
    conn: &mut SqliteConnection,
) -> Result<Vec<GroupSummary>, sqlx::Error> {
    let mut out = vec![];
    for g in groups(conn).await? {
        let members: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM member WHERE organisation_id = ?")
                .bind(g.id)
                .fetch_one(&mut *conn)
                .await?;
        let d = dashboard(conn, g).await?;
        out.push(GroupSummary {
            group: d.group,
            members,
            revenue: d.revenue,
            material_cost: d.material_cost,
            alerts: d.alerts.len(),
        });
    }
    Ok(out)
}

async fn members_of(conn: &mut SqliteConnection, org: i64) -> Result<Vec<MemberRow>, sqlx::Error> {
    let rows: Vec<(i64, String, String, String)> = sqlx::query_as(
        "SELECT id, name, phone, role FROM member WHERE organisation_id = ? ORDER BY name",
    )
    .bind(org)
    .fetch_all(&mut *conn)
    .await?;
    Ok(rows
        .into_iter()
        .map(|(id, name, phone, role)| MemberRow {
            id,
            name,
            phone,
            role: Role::from_code(&role).unwrap_or(Role::Member),
        })
        .collect())
}

pub async fn admin_data(conn: &mut SqliteConnection) -> Result<AdminData, sqlx::Error> {
    let mut groups_out = vec![];
    for g in groups(conn).await? {
        let members = members_of(conn, g.id).await?;
        groups_out.push(GroupWithMembers { group: g, members });
    }
    let mut priced_all = priced(conn, "product", "sale").await?;
    priced_all.extend(priced(conn, "material", "material").await?);
    Ok(AdminData {
        groups: groups_out,
        ase_members: members_of(conn, ledger::ORG_ASE).await?,
        priced: priced_all,
    })
}
