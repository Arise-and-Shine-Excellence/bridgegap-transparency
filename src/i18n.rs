//! English and Swahili text.
//!
//! Every string the interface shows lives in the `texts!` table below, with
//! English and Swahili side by side. Adding a key means writing both, or the
//! code does not compile.
//!
//! Which language a page uses:
//! 1. the `lang` cookie, set from the language buttons on the "More" tab;
//! 2. otherwise the `DEFAULT_LANG` environment variable (`en` or `sw`);
//! 3. otherwise English in debug builds (`cargo leptos watch`) and Swahili in
//!    release builds (what the groups use).
//!
//! The server picks the language and writes it into `<html lang>`; the browser
//! reads it back from there when it hydrates, so both always agree.
//!
//! The Swahili needs a review by a native speaker (Lujo) before launch.

use crate::error::{AppError, Field};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Lang {
    En,
    #[default]
    Sw,
}

impl Lang {
    pub fn code(self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::Sw => "sw",
        }
    }

    pub fn from_code(code: &str) -> Option<Lang> {
        match code.trim() {
            "en" => Some(Lang::En),
            "sw" => Some(Lang::Sw),
            _ => None,
        }
    }

    pub fn t(self) -> &'static Texts {
        match self {
            Lang::En => &EN,
            Lang::Sw => &SW,
        }
    }

    pub fn month(self, month: u32) -> &'static str {
        const EN_M: [&str; 12] = [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ];
        const SW_M: [&str; 12] = [
            "Januari", "Februari", "Machi", "Aprili", "Mei", "Juni", "Julai", "Agosti", "Septemba",
            "Oktoba", "Novemba", "Desemba",
        ];
        let i = (month.clamp(1, 12) - 1) as usize;
        match self {
            Lang::En => EN_M[i],
            Lang::Sw => SW_M[i],
        }
    }
}

/// The language this page is rendered in. Provided as context by `App`.
pub fn use_lang() -> Lang {
    leptos::prelude::use_context::<Lang>().unwrap_or_default()
}

pub fn use_t() -> &'static Texts {
    use_lang().t()
}

/// Server: language for the current request (cookie, then default).
#[cfg(feature = "ssr")]
pub fn request_lang() -> Lang {
    leptos::prelude::use_context::<http::request::Parts>()
        .and_then(|parts| crate::server::auth::cookie(&parts.headers, "lang"))
        .and_then(|v| Lang::from_code(&v))
        .unwrap_or_else(default_lang)
}

#[cfg(feature = "ssr")]
pub fn default_lang() -> Lang {
    std::env::var("DEFAULT_LANG")
        .ok()
        .and_then(|v| Lang::from_code(&v))
        .unwrap_or(if cfg!(debug_assertions) {
            Lang::En
        } else {
            Lang::Sw
        })
}

/// Browser: read back what the server put in `<html lang>`.
#[cfg(not(feature = "ssr"))]
pub fn request_lang() -> Lang {
    #[cfg(feature = "hydrate")]
    {
        web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.document_element())
            .and_then(|e| e.get_attribute("lang"))
            .and_then(|l| Lang::from_code(&l))
            .unwrap_or_default()
    }
    #[cfg(not(feature = "hydrate"))]
    {
        Lang::default()
    }
}

/// Replace `{0}`, `{1}`, ... in a template.
pub fn fill(template: &str, args: &[&str]) -> String {
    let mut out = template.to_string();
    for (i, a) in args.iter().enumerate() {
        out = out.replace(&format!("{{{i}}}"), a);
    }
    out
}

pub fn error_message(t: &Texts, err: &AppError) -> String {
    match err {
        AppError::NotLoggedIn => t.err_not_logged_in.into(),
        AppError::Forbidden => t.err_forbidden.into(),
        AppError::WrongPin => t.err_wrong_pin.into(),
        AppError::TooManyAttempts => t.err_too_many.into(),
        AppError::ChooseGroup => t.err_choose_group.into(),
        AppError::Invalid(f) => fill(t.err_invalid, &[field_name(t, *f)]),
        AppError::InsufficientStock { available } => fill(t.err_stock, &[available]),
        AppError::NoPrice => t.err_no_price.into(),
        AppError::AlreadySold => t.err_already_sold.into(),
        AppError::PhoneTaken => t.err_phone_taken.into(),
        AppError::NameTaken => t.err_name_taken.into(),
        AppError::Internal => t.err_internal.into(),
        AppError::ServerFn(_) => t.err_network.into(),
    }
}

fn field_name(t: &Texts, f: Field) -> &'static str {
    match f {
        Field::Product => t.product,
        Field::Quantity => t.quantity,
        Field::Date => t.date,
        Field::Name => t.name,
        Field::Phone => t.phone,
        Field::Pin => t.pin,
        Field::Price => t.price,
        Field::Role => t.role,
        Field::Group => t.group,
    }
}

macro_rules! texts {
    ($($key:ident: $en:expr, $sw:expr;)*) => {
        pub struct Texts { $(pub $key: &'static str,)* }
        pub static EN: Texts = Texts { $($key: $en,)* };
        pub static SW: Texts = Texts { $($key: $sw,)* };
    };
}

texts! {
    // key:              English,                                   Swahili
    // Navigation
    nav_home:            "Home",                                    "Nyumbani";
    nav_production:      "Production",                              "Uzalishaji";
    nav_sales:           "Sales",                                   "Mauzo";
    nav_more:            "More",                                    "Zaidi";

    // Common
    save:                "Save",                                    "Hifadhi";
    saving:              "Saving…",                                 "Inahifadhi…";
    loading:             "Loading…",                                "Inapakia…";
    no_records:          "No records yet.",                         "Bado hakuna rekodi.";
    this_month:          "This month",                              "Mwezi huu";
    total:               "Total",                                   "Jumla";
    date:                "Date",                                    "Tarehe";
    comment:             "Comment",                                 "Maoni";
    optional:            "optional",                                "si lazima";
    product:             "Product",                                 "Bidhaa";
    quantity:            "Quantity",                                "Kiasi";
    price:               "Price",                                   "Bei";
    per:                 "per",                                     "kwa";
    group:               "Group",                                   "Kikundi";
    groups:              "Groups",                                  "Vikundi";
    choose_group:        "Choose a group",                          "Chagua kikundi";
    all_groups:          "All groups",                              "Vikundi vyote";
    reversed:            "Undone",                                  "Imebatilishwa";
    undo:                "Undo",                                    "Batilisha";
    undo_explain:        "This adds a correcting entry. Nothing is deleted.", "Hii inaongeza rekodi ya kusahihisha. Hakuna kinachofutwa.";
    undo_confirm:        "Yes, undo",                               "Ndiyo, batilisha";
    recorded_by:         "Recorded by",                             "Imerekodiwa na";
    name:                "Name",                                    "Jina";
    role:                "Role",                                    "Nafasi";
    back:                "Back",                                    "Rudi";

    // Login
    login_title:         "Log in",                                  "Ingia";
    login_intro:         "Bookkeeping for our soap and cream groups.", "Daftari la hesabu la vikundi vyetu vya sabuni na losheni.";
    phone:               "Phone number",                            "Namba ya simu";
    pin:                 "PIN",                                     "PIN (namba ya siri)";
    log_in:              "Log in",                                  "Ingia";
    log_out:             "Log out",                                 "Toka";

    // Home
    welcome:             "Welcome",                                 "Karibu";
    money_in:            "Sales income",                            "Mapato ya mauzo";
    money_out:           "Material costs",                          "Gharama ya malighafi";
    profit:              "Profit",                                  "Faida";
    loss:                "Loss",                                    "Hasara";
    stock_title:         "In stock",                                "Bidhaa zilizopo";
    sold_this_month:     "Sold this month",                         "Zilizouzwa mwezi huu";
    produced_this_month: "Made this month",                         "Zilizozalishwa mwezi huu";
    cost_per_unit:       "Material cost per",                       "Gharama ya malighafi kwa";
    sells_for:           "Sells for",                               "Inauzwa kwa";
    alerts_title:        "Needs attention",                         "Inahitaji uangalizi";
    alert_unit_loss:     "Losing money on {0}: the last batch cost {1} per {2} in materials, but it sells for {3}.", "Hasara kwa {0}: uzalishaji wa mwisho uligharimu {1} kwa {2} kwa malighafi, lakini inauzwa kwa {3}.";
    alert_month_loss:    "This month you spent {0} more on materials than you earned from sales.", "Mwezi huu mmetumia {0} zaidi kwa malighafi kuliko mapato ya mauzo.";
    alert_month_loss_stock: "You still have products worth {0} to sell.", "Bado mna bidhaa zenye thamani ya {0} za kuuza.";
    alert_negative_stock: "{0} stock is below zero. Check the records.", "Bidhaa {0} ziko chini ya sifuri. Kagueni rekodi.";
    all_good:            "No problems this month.",                 "Hakuna tatizo mwezi huu.";

    // Production
    production_title:    "Production",                              "Uzalishaji";
    new_batch:           "New batch",                               "Uzalishaji mpya";
    record_batch:        "Record production",                       "Rekodi uzalishaji";
    quantity_made:       "Quantity made",                           "Kiasi kilichozalishwa";
    site:                "Production site",                         "Mahali pa uzalishaji";
    materials_used:      "Materials used",                          "Malighafi zilizotumika";
    materials_hint:      "Leave empty what you did not use.",       "Acha wazi zile ambazo hamkutumia.";
    estimated_cost:      "Material cost",                           "Gharama ya malighafi";
    expected_income:     "Income if all is sold",                   "Mapato yakiuzwa yote";
    expected_profit:     "Expected profit",                         "Faida inayotarajiwa";
    warn_cost_over_price: "This costs more to make than it sells for.", "Gharama ya kuzalisha ni kubwa kuliko bei ya kuuza.";
    batch_saved:         "Production saved.",                       "Uzalishaji umehifadhiwa.";
    ingredients:         "Ingredients",                             "Malighafi";
    recent_batches:      "Recent production",                       "Uzalishaji wa karibuni";

    // Sales
    sales_title:         "Sales",                                   "Mauzo";
    record_sale:         "Record a sale",                           "Rekodi mauzo";
    quantity_sold:       "Quantity sold",                           "Kiasi kilichouzwa";
    sale_saved:          "Sale saved:",                             "Mauzo yamehifadhiwa:";
    recent_sales:        "Recent sales",                            "Mauzo ya karibuni";
    available:           "available",                               "zilizopo";

    // More
    language:            "Language",                                "Lugha";
    account:             "Your account",                            "Akaunti yako";
    prices_title:        "Fixed prices",                            "Bei maalum";
    prices_intro:        "Prices are the same across Mbulu District.", "Bei ni sawa katika Wilaya yote ya Mbulu.";
    sale_prices:         "Selling prices",                          "Bei za kuuza";
    material_prices:     "Raw material prices",                     "Bei za malighafi";
    admin:               "Administration",                          "Usimamizi";
    role_member:         "Member",                                  "Mwanachama";
    role_leader:         "Group leader",                            "Kiongozi wa kikundi";
    role_admin:          "ASE administrator",                       "Msimamizi wa ASE";
    role_viewer:         "Viewer",                                  "Mtazamaji";

    // Admin
    admin_title:         "Administration",                          "Usimamizi";
    add_group:           "Add a group",                             "Ongeza kikundi";
    group_name:          "Group name",                              "Jina la kikundi";
    add_member:          "Add a member",                            "Ongeza mwanachama";
    members:             "Members",                                 "Wanachama";
    create:              "Create",                                  "Unda";
    created:             "Created.",                                "Imeundwa.";
    change_price:        "Change a price",                          "Badilisha bei";
    new_price_tzs:       "New price (TSh)",                         "Bei mpya (TSh)";
    price_changed:       "Price changed.",                          "Bei imebadilishwa.";
    pin_hint:            "4 to 6 digits. Give it to the member in person.", "Tarakimu 4 hadi 6. Mpe mwanachama ana kwa ana.";

    // Errors
    err_not_logged_in:   "Please log in.",                          "Tafadhali ingia.";
    err_forbidden:       "You are not allowed to do this.",         "Huruhusiwi kufanya hivi.";
    err_wrong_pin:       "Wrong phone number or PIN.",              "Namba ya simu au PIN si sahihi.";
    err_too_many:        "Too many attempts. Wait 15 minutes and try again.", "Majaribio mengi mno. Subiri dakika 15 kisha ujaribu tena.";
    err_choose_group:    "Choose a group first.",                   "Chagua kikundi kwanza.";
    err_invalid:         "Check the field: {0}.",                   "Kagua sehemu hii: {0}.";
    err_stock:           "Not enough in stock. Available: {0}.",    "Hakuna bidhaa za kutosha. Zilizopo: {0}.";
    err_no_price:        "No price is set for this product.",       "Bidhaa hii haina bei.";
    err_already_sold:    "Cannot undo: some of this product has already been sold.", "Haiwezekani kubatilisha: baadhi ya bidhaa hizi zimeshauzwa.";
    err_phone_taken:     "This phone number is already registered.", "Namba hii ya simu imeshasajiliwa.";
    err_name_taken:      "This name is already used.",              "Jina hili limeshatumika.";
    err_internal:        "Something went wrong. Try again.",        "Kuna hitilafu. Jaribu tena.";
    err_network:         "Could not reach the server. Check your internet and try again.", "Imeshindikana kufikia seva. Kagua intaneti yako na ujaribu tena.";
}
