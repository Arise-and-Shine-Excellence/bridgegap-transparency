//! Server-only code: database, ledger, authentication.

pub mod auth;
pub mod db;
pub mod ledger;
pub mod queries;
pub mod seed;

use chrono::{FixedOffset, Utc};

/// Timestamps are stored as UTC RFC 3339 strings, which sort correctly as text.
pub fn now_utc() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

/// Business dates use East Africa Time (UTC+3), where the groups are.
pub fn today_eat() -> chrono::NaiveDate {
    let eat = FixedOffset::east_opt(3 * 3600).expect("valid offset");
    Utc::now().with_timezone(&eat).date_naive()
}

/// Accepts YYYY-MM-DD, no later than tomorrow (to allow for time zones).
pub fn parse_business_date(s: &str) -> Option<String> {
    let d = chrono::NaiveDate::parse_from_str(s.trim(), "%Y-%m-%d").ok()?;
    let latest = today_eat().succ_opt()?;
    let earliest = chrono::NaiveDate::from_ymd_opt(2020, 1, 1)?;
    (d >= earliest && d <= latest).then(|| d.format("%Y-%m-%d").to_string())
}
