use crate::api::get_current_user;
use crate::components::{use_group, with_group, ErrorNote, Icon, UserResource};
use crate::error::AppError;
use crate::i18n::{request_lang, use_t, Lang};
use crate::pages::*;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Html, Link, Meta, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Outlet, ParentRoute, Redirect, Route, Router, Routes},
    hooks::use_location,
    path,
};

const FONTS: &str = "https://fonts.googleapis.com/css2?family=Bricolage+Grotesque:opsz,wght@12..96,600..800&family=Instrument+Sans:wght@400..600&display=swap";

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let lang: Lang = request_lang();
    provide_context(lang);

    view! {
        <Html attr:lang=lang.code()/>
        <Title text="Arise and Shine"/>
        // Same colour as the page, so the phone's status bar blends in.
        <Meta name="theme-color" content="#fbf5e9"/>
        <Meta name="description" content="Arise and Shine bookkeeping"/>
        <Link rel="icon" type_="image/svg+xml" href="/favicon.svg"/>
        // "Add to Home Screen": opens full screen, without the browser's bars.
        <Link rel="manifest" href="/manifest.webmanifest"/>
        <Link rel="apple-touch-icon" href="/apple-touch-icon.png"/>
        <Meta name="mobile-web-app-capable" content="yes"/>
        <Meta name="apple-mobile-web-app-capable" content="yes"/>
        <Meta name="apple-mobile-web-app-title" content="Arise & Shine"/>
        <Meta name="apple-mobile-web-app-status-bar-style" content="default"/>
        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous"/>
        <Link rel="stylesheet" href=FONTS/>
        <Stylesheet id="leptos" href="/pkg/arise_shine.css"/>

        <Router>
            <Routes fallback=NotFound>
                <Route path=path!("/login") view=LoginPage/>
                <ParentRoute path=path!("") view=AppLayout>
                    <Route path=path!("") view=HomePage/>
                    <Route path=path!("production") view=ProductionPage/>
                    <Route path=path!("production/new") view=NewBatchPage/>
                    <Route path=path!("sales") view=SalesPage/>
                    <Route path=path!("more") view=MorePage/>
                    <Route path=path!("admin") view=AdminPage/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}

/// Everything behind login: the page and the bottom menu bar. No top bar,
/// so it feels like an installed app; the page title says where you are.
///
/// Data resources must be created at the top of a component, never inside a
/// `Suspend` block: the server may run a `Suspend` block more than once, and
/// the browser would then pair resources with the wrong data. So the page
/// (`<Outlet/>`) sits outside the suspense that checks the login.
#[component]
fn AppLayout() -> impl IntoView {
    let user: UserResource = Resource::new(|| (), |_| get_current_user());
    provide_context(user);
    // `.as-app` scrolls, not the page (see main.css), so the router's own
    // scroll-to-top does not reach it. Do it here on every page change.
    let scroller = NodeRef::<leptos::html::Div>::new();
    let location = use_location();
    Effect::new(move |_| {
        location.pathname.track();
        if let Some(el) = scroller.get_untracked() {
            el.set_scroll_top(0);
        }
    });
    view! {
        <div class="as-app" node_ref=scroller>
            <Suspense fallback=|| ()>
                {move || Suspend::new(async move {
                    match user.await {
                        Ok(Some(_)) => ().into_any(),
                        Ok(None) | Err(AppError::NotLoggedIn) => view! { <Redirect path="/login"/> }.into_any(),
                        Err(e) => view! { <ErrorNote error=e/> }.into_any(),
                    }
                })}
            </Suspense>
            <main class="as-main">
                <Outlet/>
            </main>
            <BottomNav/>
        </div>
    }
}

/// The menu bar at the bottom of every page.
#[component]
fn BottomNav() -> impl IntoView {
    let t = use_t();
    let location = use_location();
    let group = use_group();
    let items: [(&'static str, &'static str, &'static str); 4] = [
        ("/", "home", t.nav_home),
        ("/production", "production", t.nav_production),
        ("/sales", "sales", t.nav_sales),
        ("/more", "more", t.nav_more),
    ];
    view! {
        <nav class="as-tabbar" aria-label="Menu">
            {items
                .into_iter()
                .map(|(href, icon, label)| {
                    let active = move || {
                        let path = location.pathname.get();
                        if href == "/" {
                            path == "/"
                        } else {
                            path.starts_with(href) || (href == "/more" && path.starts_with("/admin"))
                        }
                    };
                    view! {
                        <a
                            class="as-tabbar__item"
                            href=move || with_group(href, group.get())
                            aria-current=move || active().then_some("page")
                        >
                            <Icon name=icon/>
                            <span>{label}</span>
                        </a>
                    }
                })
                .collect_view()}
        </nav>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        if let Some(res) = use_context::<leptos_axum::ResponseOptions>() {
            res.set_status(http::StatusCode::NOT_FOUND);
        }
    }
    view! {
        <main class="as-main as-center">
            <h1 class="as-h1">"404"</h1>
            <a class="as-btn as-btn--secondary" href="/">{use_t().nav_home}</a>
        </main>
    }
}
