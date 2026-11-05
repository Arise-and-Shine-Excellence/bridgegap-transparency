//! First-run setup and demo data.

use super::{auth, ledger};
use crate::model::Role;
use sqlx::SqlitePool;

/// Create the first ASE administrator from `ADMIN_PHONE`, `ADMIN_PIN` and
/// `ADMIN_NAME` if nobody with that phone number exists yet.
pub async fn bootstrap_admin(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let (Ok(phone), Ok(pin)) = (std::env::var("ADMIN_PHONE"), std::env::var("ADMIN_PIN")) else {
        return Ok(());
    };
    let phone = auth::normalize_phone(&phone).ok_or("ADMIN_PHONE is not a valid phone number")?;
    if !auth::valid_pin(&pin) {
        return Err("ADMIN_PIN must be 4 to 6 digits".into());
    }
    let exists: Option<i64> = sqlx::query_scalar("SELECT id FROM member WHERE phone = ?")
        .bind(&phone)
        .fetch_optional(pool)
        .await?;
    if exists.is_none() {
        let name = std::env::var("ADMIN_NAME").unwrap_or_else(|_| "ASE admin".into());
        auth::create_member(pool, ledger::ORG_ASE, &name, &phone, &pin, Role::Admin).await?;
        tracing::info!(%phone, "created ASE administrator");
    }
    Ok(())
}

/// With `SEED_DEMO=1`: a demo group with a leader, a member and some records.
/// Logins (PIN 1234): leader 0700000001, member 0700000002.
pub async fn demo(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var("SEED_DEMO").map(|v| v != "1").unwrap_or(true) {
        return Ok(());
    }
    let exists: Option<i64> =
        sqlx::query_scalar("SELECT id FROM organisation WHERE name = 'Amka Kumekucha'")
            .fetch_optional(pool)
            .await?;
    if exists.is_some() {
        return Ok(());
    }
    let group =
        sqlx::query("INSERT INTO organisation (name, kind) VALUES ('Amka Kumekucha', 'group')")
            .execute(pool)
            .await?
            .last_insert_rowid();
    sqlx::query("INSERT INTO organisation (name, kind) VALUES ('Mkombozi', 'group')")
        .execute(pool)
        .await?;
    let leader = auth::create_member(
        pool,
        group,
        "Mama Neema",
        "0700000001",
        "1234",
        Role::Leader,
    )
    .await?;
    let member =
        auth::create_member(pool, group, "Rehema", "0700000002", "1234", Role::Member).await?;

    let today = super::today_eat();
    let day = |n: i64| {
        (today - chrono::Duration::days(n))
            .format("%Y-%m-%d")
            .to_string()
    };
    ledger::record_batch(
        pool,
        ledger::BatchInput {
            key: "demo-batch-1".into(),
            organisation_id: group,
            product_id: 2,
            quantity: 60_000,
            produced_on: day(3),
            site: "Dongobesh".into(),
            comment: "Production went well.".into(),
            // 60 L of liquid soap
            ingredients: vec![
                (10, 3000),
                (11, 3000),
                (12, 600),
                (13, 1500),
                (14, 600),
                (15, 150),
                (16, 30),
            ],
            by: leader,
        },
    )
    .await?;
    for (i, (litres, by)) in [(5_000, member), (12_000, leader), (3_500, member)]
        .into_iter()
        .enumerate()
    {
        ledger::record_sale(
            pool,
            ledger::SaleInput {
                key: format!("demo-sale-{i}"),
                organisation_id: group,
                product_id: 2,
                quantity: litres,
                sold_on: day(2 - i as i64),
                comment: String::new(),
                by,
            },
        )
        .await?;
    }
    tracing::info!("seeded demo group Amka Kumekucha");
    Ok(())
}
