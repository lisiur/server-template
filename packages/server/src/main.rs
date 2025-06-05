use app::App;
use axum::{Router, routing::get};
use info::Info;
use result::ServerResult;
use sea_orm::{Database, DatabaseConnection};
use settings::Settings;
use state::AppState;
use tower_cookies::CookieManagerLayer;
use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

mod auth;
mod dto;
mod error;
mod group;
mod info;
mod permission;
mod rest;
mod result;
mod role;
mod session;
mod settings;
mod state;
mod user;

#[tokio::main]
async fn main() -> ServerResult<()> {
    // Load .env
    dotenvy::dotenv().ok();

    // Init settings
    let setting = Settings::init()?;

    // Set RUST_LOG
    if std::env::var("RUST_LOG").is_err() {
        if let Some(true) = setting.debug {
            unsafe {
                std::env::set_var("RUST_LOG", "debug");
            }
        } else {
            unsafe {
                std::env::set_var("RUST_LOG", "info");
            }
        }
    }

    // Init logger
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Print info
    println!("{}", Info);

    // Connect database
    let db_conn: DatabaseConnection = Database::connect(&setting.database_url).await?;

    // Init app
    App::init(db_conn.clone()).await?;

    #[derive(OpenApi)]
    #[openapi(
        info(description = "OpenApi Docs"),
        nest(
            (path = "/auth", api = auth::router::ApiDoc, tags = ["Auth"]),
            (path = "/groups", api = group::router::ApiDoc, tags = ["Group"]),
            (path = "/permissions", api = permission::router::ApiDoc, tags = ["Permission"]),
            (path = "/roles", api = role::router::ApiDoc, tags = ["Role"]),
            (path = "/session", api = session::router::ApiDoc, tags = ["Session"]),
            (path = "/users", api = user::router::ApiDoc, tags = ["User"]),
        )
    )]
    struct ApiDoc;

    // Init router
    let app_state = AppState { db_conn };
    let router = Router::new()
        .route(
            "/health",
            get(|| async { (axum::http::StatusCode::OK, "OK") }),
        )
        .route(
            "/openapi.json",
            get(|| async { axum::Json(ApiDoc::openapi()) }),
        )
        .merge(Scalar::with_url("/docs", ApiDoc::openapi()))
        .nest("/auth", auth::router::init())
        .nest("/groups", group::router::init())
        .nest("/permissions", permission::router::init())
        .nest("/roles", role::router::init())
        .nest("/session", session::router::init())
        .nest("/users", user::router::init())
        .layer(CookieManagerLayer::new())
        .with_state(app_state);

    let server_port = setting.server_port;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", server_port))
        .await
        .unwrap();

    // Start server
    axum::serve(listener, router)
        .await
        .map_err(|err| anyhow::anyhow!(err))?;

    Ok(())
}
