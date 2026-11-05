//! The one error type every server function returns.
//!
//! Errors are codes, not sentences: the UI turns them into text in the
//! viewer's language (see `i18n::error_message`).

use leptos::server_fn::{
    codec::JsonEncoding,
    error::{FromServerFnError, ServerFnErrorErr},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Field {
    Product,
    Quantity,
    Date,
    Name,
    Phone,
    Pin,
    Price,
    Role,
    Group,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, thiserror::Error)]
pub enum AppError {
    #[error("not logged in")]
    NotLoggedIn,
    #[error("not allowed")]
    Forbidden,
    #[error("wrong phone number or PIN")]
    WrongPin,
    #[error("too many login attempts")]
    TooManyAttempts,
    #[error("choose a group first")]
    ChooseGroup,
    #[error("invalid {0:?}")]
    Invalid(Field),
    #[error("not enough stock, {available} available")]
    InsufficientStock { available: String },
    #[error("no price is set for this product")]
    NoPrice,
    #[error("cannot undo: the product has already been sold")]
    AlreadySold,
    #[error("phone number already registered")]
    PhoneTaken,
    #[error("name already used")]
    NameTaken,
    #[error("something went wrong on the server")]
    Internal,
    #[error("server function error: {0}")]
    ServerFn(String),
}

impl FromServerFnError for AppError {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        AppError::ServerFn(value.to_string())
    }
}

#[cfg(feature = "ssr")]
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        tracing::error!(error = %err, "database error");
        AppError::Internal
    }
}
