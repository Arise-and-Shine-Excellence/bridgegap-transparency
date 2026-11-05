//! Types shared by the server and the browser.

use crate::i18n::Lang;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    Member,
    Leader,
    Admin,
    Viewer,
}

impl Role {
    pub fn code(self) -> &'static str {
        match self {
            Role::Member => "member",
            Role::Leader => "leader",
            Role::Admin => "admin",
            Role::Viewer => "viewer",
        }
    }

    pub fn from_code(s: &str) -> Option<Role> {
        match s {
            "member" => Some(Role::Member),
            "leader" => Some(Role::Leader),
            "admin" => Some(Role::Admin),
            "viewer" => Some(Role::Viewer),
            _ => None,
        }
    }

    pub fn label(self, lang: Lang) -> &'static str {
        let t = lang.t();
        match self {
            Role::Member => t.role_member,
            Role::Leader => t.role_leader,
            Role::Admin => t.role_admin,
            Role::Viewer => t.role_viewer,
        }
    }

    /// ASE staff and viewers see every group; members and leaders see their own.
    pub fn sees_all_groups(self) -> bool {
        matches!(self, Role::Admin | Role::Viewer)
    }

    pub fn can_record(self) -> bool {
        !matches!(self, Role::Viewer)
    }

    pub fn can_undo(self) -> bool {
        matches!(self, Role::Leader | Role::Admin)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub phone: String,
    pub role: Role,
    pub organisation_id: i64,
    pub organisation_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupRef {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Asset {
    pub id: i64,
    pub code: String,
    pub kind: String,
    pub factor: i64,
    pub unit_en: String,
    pub unit_sw: String,
    pub name_en: String,
    pub name_sw: String,
}

impl Asset {
    pub fn name(&self, lang: Lang) -> &str {
        match lang {
            Lang::En => &self.name_en,
            Lang::Sw => &self.name_sw,
        }
    }

    pub fn unit(&self, lang: Lang) -> &str {
        match lang {
            Lang::En => &self.unit_en,
            Lang::Sw => &self.unit_sw,
        }
    }

    /// Decimal places people may type, e.g. 3 for litres stored as ml.
    pub fn decimals(&self) -> u32 {
        let mut f = self.factor;
        let mut d = 0;
        while f > 1 && f % 10 == 0 {
            f /= 10;
            d += 1;
        }
        d
    }

    /// Base units to display units: 1500 ml -> "1.5".
    pub fn fmt_qty(&self, base: i64) -> String {
        fmt_decimal(base, self.factor, self.decimals())
    }

    pub fn fmt_qty_unit(&self, base: i64, lang: Lang) -> String {
        format!("{} {}", self.fmt_qty(base), self.unit(lang))
    }

    /// Display units to base units: "1.5" L -> 1500 ml. Accepts "," as the
    /// decimal mark. Returns None for anything that is not a non-negative
    /// number with at most `decimals()` decimal places.
    pub fn parse_qty(&self, input: &str) -> Option<i64> {
        parse_decimal(input, self.factor, self.decimals())
    }

    /// Colour class for the product line, from the design system.
    pub fn line(&self) -> &'static str {
        match self.code.as_str() {
            "liquid_soap_ml" => "liquid",
            "soap_bar" => "bar",
            "lotion_bottle" => "cream",
            _ if self.kind == "material" => "farm",
            _ => "outline",
        }
    }
}

fn fmt_decimal(base: i64, factor: i64, decimals: u32) -> String {
    let sign = if base < 0 { "-" } else { "" };
    let abs = base.unsigned_abs();
    let factor = factor.max(1) as u64;
    let whole = abs / factor;
    let frac = abs % factor;
    if frac == 0 || decimals == 0 {
        return format!("{sign}{}", group_thousands(whole));
    }
    let frac = format!("{:0width$}", frac, width = decimals as usize);
    format!(
        "{sign}{}.{}",
        group_thousands(whole),
        frac.trim_end_matches('0')
    )
}

fn parse_decimal(input: &str, factor: i64, decimals: u32) -> Option<i64> {
    let s = input.trim().replace(',', ".").replace(' ', "");
    if s.is_empty() {
        return None;
    }
    let (whole, frac) = match s.split_once('.') {
        Some((w, f)) => (w, f),
        None => (s.as_str(), ""),
    };
    let whole = if whole.is_empty() { "0" } else { whole };
    if !whole.chars().all(|c| c.is_ascii_digit()) || !frac.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    if frac.len() as u32 > decimals {
        return None;
    }
    let whole: i64 = whole.parse().ok()?;
    let frac_val: i64 = if frac.is_empty() {
        0
    } else {
        let padded = format!("{:0<width$}", frac, width = decimals as usize);
        padded.parse().ok()?
    };
    let scale = factor / 10_i64.pow(decimals);
    whole.checked_mul(factor)?.checked_add(frac_val * scale)
}

fn group_thousands(n: u64) -> String {
    let s = n.to_string();
    let mut out = String::with_capacity(s.len() + s.len() / 3);
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    out
}

/// "TSh 2,000"
pub fn fmt_tzs(amount: i64) -> String {
    if amount < 0 {
        format!("-TSh {}", group_thousands(amount.unsigned_abs()))
    } else {
        format!("TSh {}", group_thousands(amount as u64))
    }
}

/// A fixed price: `amount_tzs` buys `per_quantity` base units.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Price {
    pub amount_tzs: i64,
    pub per_quantity: i64,
}

impl Price {
    /// Value of `qty` base units in whole shillings, rounded half up.
    pub fn value_of(&self, qty: i64) -> i64 {
        let num = qty as i128 * self.amount_tzs as i128;
        let den = self.per_quantity.max(1) as i128;
        ((num * 2 + den) / (den * 2)) as i64
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PricedAsset {
    pub asset: Asset,
    pub price: Option<Price>,
}

impl PricedAsset {
    /// Price of one display unit (one litre, one bar).
    pub fn price_per_unit(&self) -> Option<i64> {
        self.price.map(|p| p.value_of(self.asset.factor))
    }
}

/// Everything a production or sale form needs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormContext {
    /// Idempotency key for the next submission.
    pub key: String,
    pub today: String,
    pub group: GroupRef,
    pub products: Vec<PricedAsset>,
    pub materials: Vec<PricedAsset>,
    /// (product asset id, base units in stock)
    pub stock: Vec<(i64, i64)>,
    pub last_site: String,
}

impl FormContext {
    pub fn stock_of(&self, asset_id: i64) -> i64 {
        self.stock
            .iter()
            .find(|(id, _)| *id == asset_id)
            .map(|(_, q)| *q)
            .unwrap_or(0)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ingredient {
    pub asset: Asset,
    pub amount: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BatchRow {
    pub id: i64,
    pub product: Asset,
    pub quantity: i64,
    pub produced_on: String,
    pub site: String,
    pub comment: String,
    pub material_cost: i64,
    pub reversed: bool,
    pub recorded_by: String,
    pub ingredients: Vec<Ingredient>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaleRow {
    pub id: i64,
    pub product: Asset,
    pub quantity: i64,
    pub sold_on: String,
    pub comment: String,
    pub amount_tzs: i64,
    pub reversed: bool,
    pub recorded_by: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductLine {
    pub product: PricedAsset,
    pub in_stock: i64,
    pub sold_month: i64,
    pub produced_month: i64,
    /// Material cost per display unit in the latest batch.
    pub last_unit_cost: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Alert {
    UnitLoss {
        product: Asset,
        unit_cost: i64,
        price: i64,
    },
    MonthLoss {
        loss: i64,
        unsold_value: i64,
    },
    NegativeStock {
        product: Asset,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dashboard {
    pub group: GroupRef,
    pub year: i32,
    pub month: u32,
    pub revenue: i64,
    pub material_cost: i64,
    pub products: Vec<ProductLine>,
    pub alerts: Vec<Alert>,
}

impl Dashboard {
    pub fn profit(&self) -> i64 {
        self.revenue - self.material_cost
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupSummary {
    pub group: GroupRef,
    pub members: i64,
    pub revenue: i64,
    pub material_cost: i64,
    pub alerts: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberRow {
    pub id: i64,
    pub name: String,
    pub phone: String,
    pub role: Role,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupWithMembers {
    pub group: GroupRef,
    pub members: Vec<MemberRow>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdminData {
    pub groups: Vec<GroupWithMembers>,
    pub ase_members: Vec<MemberRow>,
    pub priced: Vec<PricedAsset>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn litres() -> Asset {
        Asset {
            id: 2,
            code: "liquid_soap_ml".into(),
            kind: "product".into(),
            factor: 1000,
            unit_en: "L".into(),
            unit_sw: "L".into(),
            name_en: "Liquid soap".into(),
            name_sw: "Sabuni ya maji".into(),
        }
    }

    #[test]
    fn parses_display_units() {
        let a = litres();
        assert_eq!(a.parse_qty("1.5"), Some(1500));
        assert_eq!(a.parse_qty("1,5"), Some(1500));
        assert_eq!(a.parse_qty("60"), Some(60_000));
        assert_eq!(a.parse_qty(".25"), Some(250));
        assert_eq!(a.parse_qty("0.0005"), None);
        assert_eq!(a.parse_qty("-1"), None);
        assert_eq!(a.parse_qty("abc"), None);
        assert_eq!(a.parse_qty(""), None);
    }

    #[test]
    fn formats_display_units() {
        let a = litres();
        assert_eq!(a.fmt_qty(1500), "1.5");
        assert_eq!(a.fmt_qty(60_000), "60");
        assert_eq!(a.fmt_qty(1_250_000), "1,250");
        assert_eq!(a.fmt_qty(-500), "-0.5");
    }

    #[test]
    fn prices_round_half_up() {
        let p = Price {
            amount_tzs: 2000,
            per_quantity: 1000,
        };
        assert_eq!(p.value_of(1500), 3000);
        let p = Price {
            amount_tzs: 1000,
            per_quantity: 3,
        };
        assert_eq!(p.value_of(1), 333);
        assert_eq!(p.value_of(2), 667);
    }

    #[test]
    fn formats_money() {
        assert_eq!(fmt_tzs(2000), "TSh 2,000");
        assert_eq!(fmt_tzs(-1_234_567), "-TSh 1,234,567");
        assert_eq!(fmt_tzs(0), "TSh 0");
    }
}
