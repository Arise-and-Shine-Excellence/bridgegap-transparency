#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use arise_shine::app::{shell, App};
    use arise_shine::server::{db, seed};
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    // Local development settings; in production use real environment variables.
    let _ = dotenvy::dotenv();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,sqlx=warn".into()),
        )
        .init();

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://arise_shine.db".into());
    let pool = db::connect(&database_url).await.expect("open database");
    seed::bootstrap_admin(&pool).await.expect("create admin");
    seed::demo(&pool).await.expect("seed demo data");

    let conf = get_configuration(None).expect("leptos configuration");
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            {
                let pool = pool.clone();
                move || provide_context(pool.clone())
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options)
        // gzip/brotli: matters on slow mobile connections (the WASM bundle shrinks ~4x).
        .layer(tower_http::compression::CompressionLayer::new());

    tracing::info!("listening on http://{addr} (database {database_url})");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("bind address");
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await
        .expect("server");
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // The browser entry point is `hydrate()` in lib.rs.
}
