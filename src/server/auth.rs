//! Phone number + PIN login, sessions and rate limiting.

use crate::error::AppError;
use crate::model::{Role, User};
use argon2::password_hash::{
    rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
};
use argon2::Argon2;
use http::HeaderMap;
use rand::RngCore;
use sha2::{Digest, Sha256};
use sqlx::SqlitePool;

pub const SESSION_COOKIE: &str = "as_session";
const SESSION_DAYS: i64 = 30;
const MAX_FAILURES: i64 = 5;
const FAILURE_WINDOW_MINUTES: i64 = 15;

pub fn hash_pin(pin: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(pin.as_bytes(), &salt)
        .expect("argon2 hashing")
        .to_string()
}

pub fn verify_pin(pin: &str, hash: &str) -> bool {
    PasswordHash::new(hash)
        .map(|h| {
            Argon2::default()
                .verify_password(pin.as_bytes(), &h)
                .is_ok()
        })
        .unwrap_or(false)
}

pub fn valid_pin(pin: &str) -> bool {
    (4..=6).contains(&pin.len()) && pin.chars().all(|c| c.is_ascii_digit())
}

/// Normalise Tanzanian mobile numbers to the local form `07xxxxxxxx`.
/// Accepts `+255 7xx xxx xxx`, `2557xxxxxxxx`, `07xxxxxxxx` and `7xxxxxxxx`.
pub fn normalize_phone(input: &str) -> Option<String> {
    let digits: String = input.chars().filter(|c| c.is_ascii_digit()).collect();
    let local = if digits.len() == 12 && digits.starts_with("255") {
        format!("0{}", &digits[3..])
    } else if digits.len() == 9 && !digits.starts_with('0') {
        format!("0{digits}")
    } else {
        digits
    };
    (local.len() == 10 && local.starts_with('0')).then_some(local)
}

pub fn cookie(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get_all(http::header::COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(|v| v.split(';'))
        .filter_map(|pair| pair.trim().split_once('='))
        .find(|(k, _)| *k == name)
        .map(|(_, v)| v.to_string())
}

fn sha256_hex(s: &str) -> String {
    Sha256::digest(s.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn secure_flag() -> &'static str {
    if std::env::var("COOKIE_SECURE")
        .map(|v| v == "1" || v == "true")
        .unwrap_or(false)
    {
        "; Secure"
    } else {
        ""
    }
}

pub fn session_cookie(token: &str) -> String {
    format!(
        "{SESSION_COOKIE}={token}; Path=/; HttpOnly; SameSite=Lax; Max-Age={}{}",
        SESSION_DAYS * 86_400,
        secure_flag()
    )
}

pub fn clear_session_cookie() -> String {
    format!(
        "{SESSION_COOKIE}=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0{}",
        secure_flag()
    )
}

pub fn lang_cookie(code: &str) -> String {
    format!(
        "lang={code}; Path=/; SameSite=Lax; Max-Age=31536000{}",
        secure_flag()
    )
}

fn minutes_ago(m: i64) -> String {
    (chrono::Utc::now() - chrono::Duration::minutes(m))
        .format("%Y-%m-%dT%H:%M:%SZ")
        .to_string()
}

/// Check a phone + PIN, with at most `MAX_FAILURES` failures per phone number
/// per `FAILURE_WINDOW_MINUTES`. Returns the member id.
pub async fn login(pool: &SqlitePool, phone: &str, pin: &str) -> Result<i64, AppError> {
    let phone = normalize_phone(phone).ok_or(AppError::WrongPin)?;
    let failures: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM login_attempt WHERE phone = ? AND success = 0 AND at > ?",
    )
    .bind(&phone)
    .bind(minutes_ago(FAILURE_WINDOW_MINUTES))
    .fetch_one(pool)
    .await?;
    if failures >= MAX_FAILURES {
        return Err(AppError::TooManyAttempts);
    }

    let row: Option<(i64, String)> =
        sqlx::query_as("SELECT id, pin_hash FROM member WHERE phone = ?")
            .bind(&phone)
            .fetch_optional(pool)
            .await?;
    let ok = row.as_ref().is_some_and(|(_, hash)| verify_pin(pin, hash));
    sqlx::query("INSERT INTO login_attempt (phone, at, success) VALUES (?, ?, ?)")
        .bind(&phone)
        .bind(super::now_utc())
        .bind(ok)
        .execute(pool)
        .await?;
    match row {
        Some((id, _)) if ok => Ok(id),
        _ => Err(AppError::WrongPin),
    }
}

pub async fn create_session(pool: &SqlitePool, member_id: i64) -> Result<String, AppError> {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    let token: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
    let expires = (chrono::Utc::now() + chrono::Duration::days(SESSION_DAYS))
        .format("%Y-%m-%dT%H:%M:%SZ")
        .to_string();
    sqlx::query("INSERT INTO session (token_hash, member_id, expires_at) VALUES (?, ?, ?)")
        .bind(sha256_hex(&token))
        .bind(member_id)
        .bind(expires)
        .execute(pool)
        .await?;
    Ok(token)
}

pub async fn delete_session(pool: &SqlitePool, token: &str) -> Result<(), AppError> {
    sqlx::query("DELETE FROM session WHERE token_hash = ?")
        .bind(sha256_hex(token))
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn user_for_token(pool: &SqlitePool, token: &str) -> Result<Option<User>, AppError> {
    let row: Option<(i64, String, String, String, i64, String)> = sqlx::query_as(
        "SELECT m.id, m.name, m.phone, m.role, o.id, o.name
         FROM session s
         JOIN member m ON m.id = s.member_id
         JOIN organisation o ON o.id = m.organisation_id
         WHERE s.token_hash = ? AND s.expires_at > ?",
    )
    .bind(sha256_hex(token))
    .bind(super::now_utc())
    .fetch_optional(pool)
    .await?;
    Ok(row.map(
        |(id, name, phone, role, organisation_id, organisation_name)| User {
            id,
            name,
            phone,
            role: Role::from_code(&role).unwrap_or(Role::Member),
            organisation_id,
            organisation_name,
        },
    ))
}

pub async fn create_member(
    pool: &SqlitePool,
    organisation_id: i64,
    name: &str,
    phone: &str,
    pin: &str,
    role: Role,
) -> Result<i64, AppError> {
    let res = sqlx::query(
        "INSERT INTO member (organisation_id, name, phone, pin_hash, role) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(organisation_id)
    .bind(name)
    .bind(phone)
    .bind(hash_pin(pin))
    .bind(role.code())
    .execute(pool)
    .await;
    match res {
        Ok(r) => Ok(r.last_insert_rowid()),
        Err(sqlx::Error::Database(e)) if e.is_unique_violation() => Err(AppError::PhoneTaken),
        Err(e) => Err(e.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalises_phone_numbers() {
        assert_eq!(
            normalize_phone("+255 712 345 678").as_deref(),
            Some("0712345678")
        );
        assert_eq!(
            normalize_phone("255712345678").as_deref(),
            Some("0712345678")
        );
        assert_eq!(
            normalize_phone("0712 345 678").as_deref(),
            Some("0712345678")
        );
        assert_eq!(normalize_phone("712345678").as_deref(), Some("0712345678"));
        assert_eq!(normalize_phone("12345"), None);
    }

    #[test]
    fn hashes_and_verifies_pins() {
        let h = hash_pin("1234");
        assert!(verify_pin("1234", &h));
        assert!(!verify_pin("4321", &h));
        assert!(valid_pin("123456"));
        assert!(!valid_pin("12a4"));
        assert!(!valid_pin("123"));
    }

    #[tokio::test]
    async fn rate_limits_failed_logins() {
        let pool = crate::server::db::test_pool().await;
        create_member(&pool, 1, "Lujo", "0700000000", "1234", Role::Admin)
            .await
            .unwrap();
        for _ in 0..MAX_FAILURES {
            assert_eq!(
                login(&pool, "0700000000", "0000").await,
                Err(AppError::WrongPin)
            );
        }
        // Even the right PIN is refused while locked out.
        assert_eq!(
            login(&pool, "0700000000", "1234").await,
            Err(AppError::TooManyAttempts)
        );
    }
}
