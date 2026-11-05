//! Server functions: the only way the browser talks to the server.
//!
//! Each one works as a plain HTML form POST before the WASM has loaded, and
//! as a fetch call after.

use crate::error::AppError;
use crate::model::*;
use leptos::prelude::*;
use std::collections::HashMap;

#[cfg(feature = "ssr")]
mod ssr {
    pub use crate::error::{AppError, Field};
    pub use crate::model::{Role, User};
    pub use crate::server::{auth, ledger, queries};
    pub use leptos::prelude::*;
    pub use sqlx::SqlitePool;

    pub fn pool() -> Result<SqlitePool, AppError> {
        use_context::<SqlitePool>().ok_or(AppError::Internal)
    }

    pub fn set_cookie(value: String) {
        if let (Some(res), Ok(v)) = (
            use_context::<leptos_axum::ResponseOptions>(),
            http::HeaderValue::from_str(&value),
        ) {
            res.append_header(http::header::SET_COOKIE, v);
        }
    }

    pub fn session_token() -> Option<String> {
        use_context::<http::request::Parts>()
            .and_then(|p| auth::cookie(&p.headers, auth::SESSION_COOKIE))
    }

    pub async fn current_user() -> Result<Option<User>, AppError> {
        match session_token() {
            Some(token) => auth::user_for_token(&pool()?, &token).await,
            None => Ok(None),
        }
    }

    pub async fn require_user() -> Result<User, AppError> {
        current_user().await?.ok_or(AppError::NotLoggedIn)
    }

    /// Which group a request is about. Members and leaders always act for
    /// their own group; ASE staff choose one.
    pub async fn resolve_group(
        user: &User,
        group_id: Option<i64>,
    ) -> Result<crate::model::GroupRef, AppError> {
        let id = if user.role.sees_all_groups() {
            group_id.ok_or(AppError::ChooseGroup)?
        } else {
            user.organisation_id
        };
        let mut conn = pool()?.acquire().await?;
        queries::group(&mut conn, id)
            .await?
            .ok_or(AppError::ChooseGroup)
    }

    pub fn require_admin(user: &User) -> Result<(), AppError> {
        (user.role == Role::Admin)
            .then_some(())
            .ok_or(AppError::Forbidden)
    }

    pub fn clean(s: &str, max: usize) -> String {
        s.trim().chars().take(max).collect()
    }
}

#[server]
pub async fn get_current_user() -> Result<Option<User>, AppError> {
    ssr::current_user().await
}

#[server]
pub async fn login(phone: String, pin: String) -> Result<(), AppError> {
    use ssr::*;
    let pool = pool()?;
    let member_id = auth::login(&pool, &phone, pin.trim()).await?;
    let token = auth::create_session(&pool, member_id).await?;
    set_cookie(auth::session_cookie(&token));
    leptos_axum::redirect("/");
    Ok(())
}

#[server]
pub async fn logout() -> Result<(), AppError> {
    use ssr::*;
    if let Some(token) = session_token() {
        auth::delete_session(&pool()?, &token).await?;
    }
    set_cookie(auth::clear_session_cookie());
    leptos_axum::redirect("/login");
    Ok(())
}

/// Remember the language in a cookie. Submitted as a plain form, so the
/// browser reloads the page in the new language.
#[server]
pub async fn set_lang(lang: String) -> Result<(), AppError> {
    use ssr::*;
    let lang = crate::i18n::Lang::from_code(&lang).ok_or(AppError::Invalid(Field::Name))?;
    set_cookie(auth::lang_cookie(lang.code()));
    Ok(())
}

#[server]
pub async fn list_groups() -> Result<Vec<GroupRef>, AppError> {
    use ssr::*;
    let user = require_user().await?;
    let mut conn = pool()?.acquire().await?;
    if user.role.sees_all_groups() {
        Ok(queries::groups(&mut conn).await?)
    } else {
        Ok(queries::group(&mut conn, user.organisation_id)
            .await?
            .into_iter()
            .collect())
    }
}

/// Prices, stock and a fresh idempotency key for the production and sale forms.
#[server]
pub async fn get_form_context(group_id: Option<i64>) -> Result<FormContext, AppError> {
    use ssr::*;
    let user = require_user().await?;
    let group = resolve_group(&user, group_id).await?;
    let mut conn = pool()?.acquire().await?;
    Ok(FormContext {
        key: uuid::Uuid::new_v4().to_string(),
        today: crate::server::today_eat().format("%Y-%m-%d").to_string(),
        products: queries::priced(&mut conn, "product", "sale").await?,
        materials: queries::priced(&mut conn, "material", "material").await?,
        stock: queries::stock(&mut conn, group.id).await?,
        last_site: queries::last_site(&mut conn, group.id).await?,
        group,
    })
}

#[server]
#[allow(clippy::too_many_arguments)]
pub async fn record_batch(
    key: String,
    group_id: Option<i64>,
    product_id: i64,
    quantity: String,
    produced_on: String,
    site: String,
    comment: String,
    amounts: Option<HashMap<String, String>>,
) -> Result<i64, AppError> {
    use ssr::*;
    let user = require_user().await?;
    if !user.role.can_record() {
        return Err(AppError::Forbidden);
    }
    let group = resolve_group(&user, group_id).await?;
    let pool = pool()?;
    let mut conn = pool.acquire().await?;
    let product = queries::asset(&mut conn, product_id)
        .await
        .map_err(|_| AppError::Invalid(Field::Product))?;
    let quantity = product
        .parse_qty(&quantity)
        .filter(|q| *q > 0)
        .ok_or(AppError::Invalid(Field::Quantity))?;
    let produced_on =
        crate::server::parse_business_date(&produced_on).ok_or(AppError::Invalid(Field::Date))?;

    let amounts = amounts.unwrap_or_default();
    let mut ingredients = vec![];
    for material in queries::assets(&mut conn, "material").await? {
        let raw = amounts.get(&material.code).map(|s| s.trim()).unwrap_or("");
        if raw.is_empty() {
            continue;
        }
        let amount = material
            .parse_qty(raw)
            .ok_or(AppError::Invalid(Field::Quantity))?;
        if amount > 0 {
            ingredients.push((material.id, amount));
        }
    }
    drop(conn);

    let id = ledger::record_batch(
        &pool,
        ledger::BatchInput {
            key: clean(&key, 64),
            organisation_id: group.id,
            product_id,
            quantity,
            produced_on,
            site: clean(&site, 120),
            comment: clean(&comment, 1000),
            ingredients,
            by: user.id,
        },
    )
    .await?;
    let back = match group_id.filter(|_| user.role.sees_all_groups()) {
        Some(g) => format!("/production?group={g}"),
        None => "/production".to_string(),
    };
    leptos_axum::redirect(&back);
    Ok(id)
}

/// Returns the sale amount in TZS.
#[server]
pub async fn record_sale(
    key: String,
    group_id: Option<i64>,
    product_id: i64,
    quantity: String,
    sold_on: String,
    comment: Option<String>,
) -> Result<i64, AppError> {
    use ssr::*;
    let user = require_user().await?;
    if !user.role.can_record() {
        return Err(AppError::Forbidden);
    }
    let group = resolve_group(&user, group_id).await?;
    let pool = pool()?;
    let product = {
        let mut conn = pool.acquire().await?;
        queries::asset(&mut conn, product_id)
            .await
            .map_err(|_| AppError::Invalid(Field::Product))?
    };
    let quantity = product
        .parse_qty(&quantity)
        .filter(|q| *q > 0)
        .ok_or(AppError::Invalid(Field::Quantity))?;
    let sold_on =
        crate::server::parse_business_date(&sold_on).ok_or(AppError::Invalid(Field::Date))?;
    let (_, amount) = ledger::record_sale(
        &pool,
        ledger::SaleInput {
            key: clean(&key, 64),
            organisation_id: group.id,
            product_id,
            quantity,
            sold_on,
            comment: clean(comment.as_deref().unwrap_or(""), 1000),
            by: user.id,
        },
    )
    .await?;
    Ok(amount)
}

#[server]
pub async fn list_batches(group_id: Option<i64>) -> Result<Vec<BatchRow>, AppError> {
    use ssr::*;
    let user = require_user().await?;
    let group = resolve_group(&user, group_id).await?;
    let mut conn = pool()?.acquire().await?;
    Ok(queries::batches(&mut conn, group.id, 50).await?)
}

#[server]
pub async fn list_sales(group_id: Option<i64>) -> Result<Vec<SaleRow>, AppError> {
    use ssr::*;
    let user = require_user().await?;
    let group = resolve_group(&user, group_id).await?;
    let mut conn = pool()?.acquire().await?;
    Ok(queries::sales(&mut conn, group.id, 50).await?)
}

#[server]
pub async fn undo_batch(id: i64, group_id: Option<i64>) -> Result<(), AppError> {
    use ssr::*;
    let user = require_user().await?;
    if !user.role.can_undo() {
        return Err(AppError::Forbidden);
    }
    let group = resolve_group(&user, group_id).await?;
    ledger::reverse(&pool()?, ledger::Record::Batch(id), group.id, user.id).await
}

#[server]
pub async fn undo_sale(id: i64, group_id: Option<i64>) -> Result<(), AppError> {
    use ssr::*;
    let user = require_user().await?;
    if !user.role.can_undo() {
        return Err(AppError::Forbidden);
    }
    let group = resolve_group(&user, group_id).await?;
    ledger::reverse(&pool()?, ledger::Record::Sale(id), group.id, user.id).await
}

#[server]
pub async fn get_dashboard(group_id: Option<i64>) -> Result<Dashboard, AppError> {
    use ssr::*;
    let user = require_user().await?;
    let group = resolve_group(&user, group_id).await?;
    let mut conn = pool()?.acquire().await?;
    Ok(queries::dashboard(&mut conn, group).await?)
}

#[server]
pub async fn get_group_summaries() -> Result<Vec<GroupSummary>, AppError> {
    use ssr::*;
    let user = require_user().await?;
    if !user.role.sees_all_groups() {
        return Err(AppError::Forbidden);
    }
    let mut conn = pool()?.acquire().await?;
    Ok(queries::group_summaries(&mut conn).await?)
}

#[server]
pub async fn get_prices() -> Result<Vec<PricedAsset>, AppError> {
    use ssr::*;
    require_user().await?;
    let mut conn = pool()?.acquire().await?;
    let mut all = queries::priced(&mut conn, "product", "sale").await?;
    all.extend(queries::priced(&mut conn, "material", "material").await?);
    Ok(all)
}

#[server]
pub async fn get_admin_data() -> Result<AdminData, AppError> {
    use ssr::*;
    let user = require_user().await?;
    require_admin(&user)?;
    let mut conn = pool()?.acquire().await?;
    Ok(queries::admin_data(&mut conn).await?)
}

#[server]
pub async fn create_group(name: String) -> Result<(), AppError> {
    use ssr::*;
    let user = require_user().await?;
    require_admin(&user)?;
    let name = clean(&name, 80);
    if name.is_empty() {
        return Err(AppError::Invalid(Field::Name));
    }
    match sqlx::query("INSERT INTO organisation (name, kind) VALUES (?, 'group')")
        .bind(&name)
        .execute(&pool()?)
        .await
    {
        Ok(_) => Ok(()),
        Err(sqlx::Error::Database(e)) if e.is_unique_violation() => Err(AppError::NameTaken),
        Err(e) => Err(e.into()),
    }
}

/// `group_id` 0 means ASE itself (for new ASE staff and viewers).
#[server]
pub async fn create_member(
    group_id: i64,
    name: String,
    phone: String,
    pin: String,
    role: String,
) -> Result<(), AppError> {
    use ssr::*;
    let user = require_user().await?;
    require_admin(&user)?;
    let name = clean(&name, 80);
    if name.is_empty() {
        return Err(AppError::Invalid(Field::Name));
    }
    let phone = auth::normalize_phone(&phone).ok_or(AppError::Invalid(Field::Phone))?;
    if !auth::valid_pin(pin.trim()) {
        return Err(AppError::Invalid(Field::Pin));
    }
    let role = Role::from_code(&role).ok_or(AppError::Invalid(Field::Role))?;
    let org = if group_id == 0 {
        if !role.sees_all_groups() {
            return Err(AppError::Invalid(Field::Role));
        }
        ledger::ORG_ASE
    } else {
        if role.sees_all_groups() {
            return Err(AppError::Invalid(Field::Role));
        }
        let mut conn = pool()?.acquire().await?;
        queries::group(&mut conn, group_id)
            .await?
            .ok_or(AppError::Invalid(Field::Group))?
            .id
    };
    auth::create_member(&pool()?, org, &name, &phone, pin.trim(), role).await?;
    Ok(())
}

/// Set a new fixed price for one display unit (one litre, one kg, one bar).
/// Old prices stay in the table, so past records keep their history.
#[server]
pub async fn set_price(asset_id: i64, amount_tzs: String) -> Result<(), AppError> {
    use ssr::*;
    let user = require_user().await?;
    require_admin(&user)?;
    let amount: i64 = amount_tzs
        .trim()
        .replace(',', "")
        .parse()
        .ok()
        .filter(|a| *a >= 0)
        .ok_or(AppError::Invalid(Field::Price))?;
    let pool = pool()?;
    let mut conn = pool.acquire().await?;
    let asset = queries::asset(&mut conn, asset_id)
        .await
        .map_err(|_| AppError::Invalid(Field::Product))?;
    let kind = match asset.kind.as_str() {
        "product" => "sale",
        "material" => "material",
        _ => return Err(AppError::Invalid(Field::Product)),
    };
    sqlx::query("INSERT INTO price (asset_id, kind, amount_tzs, per_quantity, valid_from) VALUES (?, ?, ?, ?, ?)")
        .bind(asset.id)
        .bind(kind)
        .bind(amount)
        .bind(asset.factor)
        .bind(crate::server::now_utc())
        .execute(&mut *conn)
        .await?;
    Ok(())
}
