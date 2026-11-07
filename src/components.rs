//! Small building blocks shared by the pages.

use crate::error::AppError;
use crate::i18n::{error_message, use_t};
use crate::model::User;
use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

/// 24px line icons, 2.2 stroke with round caps (design system: Iconography).
#[component]
pub fn Icon(name: &'static str, #[prop(optional)] class: &'static str) -> impl IntoView {
    let paths: &[&str] = match name {
        "home" => &["M3 10.5 12 3l9 7.5", "M5 9v12h14V9", "M10 21v-6h4v6"],
        "production" => &[
            "M9 3h6",
            "M10 3v6l-5.6 9.7A1.5 1.5 0 0 0 5.7 21h12.6a1.5 1.5 0 0 0 1.3-2.3L14 9V3",
            "M7.5 15h9",
        ],
        "sales" => &["M5 9h14l-1.4 10.2a1 1 0 0 1-1 .8H7.4a1 1 0 0 1-1-.8Z", "M9 9l3-5 3 5"],
        "more" => &["M4 6h16", "M4 12h16", "M4 18h16"],
        "alert" => &["M12 3 2 20h20L12 3Z", "M12 10v4", "M12 17h.01"],
        "check" => &["M5 12.5 10 17l9-10"],
        "plus" => &["M12 5v14", "M5 12h14"],
        "back" => &["M15 6l-6 6 6 6"],
        "next" => &["M9 6l6 6-6 6"],
        _ => &[],
    };
    view! {
        <svg class=format!("as-icon {class}") viewBox="0 0 24 24" fill="none" stroke="currentColor"
            stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
            <path d=paths.join(" ")></path>
        </svg>
    }
}

/// The selected group for ASE staff, from `?group=<id>`.
pub fn use_group() -> Memo<Option<i64>> {
    let query = use_query_map();
    Memo::new(move |_| query.read().get("group").and_then(|g| g.parse().ok()))
}

/// Append `?group=<id>` to a path when a group is selected.
pub fn with_group(path: &str, group: Option<i64>) -> String {
    match group {
        Some(g) => format!("{path}?group={g}"),
        None => path.to_string(),
    }
}

#[component]
pub fn GroupHidden(group: Option<i64>) -> impl IntoView {
    group.map(|g| view! { <input type="hidden" name="group_id" value=g.to_string()/> })
}

#[component]
pub fn ErrorNote(error: AppError) -> impl IntoView {
    let t = use_t();
    view! {
        <p class="as-note as-note--danger" role="alert">
            <Icon name="alert"/>
            <span>{error_message(t, &error)}</span>
        </p>
    }
}

#[component]
pub fn SuccessNote(children: Children) -> impl IntoView {
    view! {
        <p class="as-note as-note--success" role="status">
            <Icon name="check"/>
            <span>{children()}</span>
        </p>
    }
}

#[component]
pub fn Loading() -> impl IntoView {
    let t = use_t();
    view! { <p class="as-loading" aria-live="polite">{t.loading}</p> }
}

#[component]
pub fn PageTitle(
    title: &'static str,
    #[prop(optional, into)] eyebrow: Option<String>,
) -> impl IntoView {
    view! {
        <header class="as-page-title">
            {eyebrow.map(|e| view! { <p class="as-eyebrow">{e}</p> })}
            <h1>{title}</h1>
        </header>
    }
}

/// The logged-in user, loaded once by the app layout.
pub type UserResource = Resource<Result<Option<User>, AppError>>;

pub fn use_user() -> UserResource {
    expect_context::<UserResource>()
}

/// Wait for the logged-in user inside a `Suspend` block.
pub async fn current_user(user: UserResource) -> Result<User, AppError> {
    user.await.and_then(|u| u.ok_or(AppError::NotLoggedIn))
}

/// Show an error. ASE staff who have not chosen a group are sent to the
/// overview on the home page, which lists every group.
pub fn error_or_picker(error: AppError) -> AnyView {
    if error == AppError::ChooseGroup {
        let t = use_t();
        view! {
            <a class="as-list__link" href="/">
                <span>{t.choose_group}</span>
                <Icon name="next"/>
            </a>
        }
        .into_any()
    } else {
        view! { <ErrorNote error=error/> }.into_any()
    }
}
