mod admin;
mod home;
mod login;
mod more;
mod production;
mod sales;

pub use admin::AdminPage;
pub use home::HomePage;
pub use login::LoginPage;
pub use more::{LanguageSwitch, MorePage};
pub use production::{NewBatchPage, ProductionPage};
pub use sales::SalesPage;
