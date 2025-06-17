use app::App;
use axum::{Extension, Router, routing::get};
use info::Info;
use result::ServerResult;
use sea_orm::{Database, DatabaseConnection};
use settings::Settings;
use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

mod api_router;
mod dto;
mod error;
mod extractors;
mod info;
mod middlewares;
mod response;
mod result;
mod routes;
mod settings;
mod utils;

#[tokio::main]
async fn main() -> ServerResult<()> {
    // Load .env
    println!("Loading .env...");
    dotenvy::dotenv().ok();

    // Init settings
    println!("Init settings...");
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
    println!("Init logger...");
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Connect database
    println!("Connecting database...");
    let db_conn: DatabaseConnection = Database::connect(&setting.database_url).await?;

    // Init app
    println!("Apply migrations...");
    App::init(db_conn.clone()).await?;

    #[derive(OpenApi)]
    #[openapi(
        info(description = "OpenApi Docs"),
        nest(
            (path = "/auth", api = routes::auth::router::ApiDoc, tags = ["Auth"]),
            (path = "/department", api = routes::department::router::ApiDoc, tags = ["Department"]),
            (path = "/groups", api = routes::group::router::ApiDoc, tags = ["Group"]),
            (path = "/permissions", api = routes::permission::router::ApiDoc, tags = ["Permission"]),
            (path = "/roles", api = routes::role::router::ApiDoc, tags = ["Role"]),
            (path = "/session", api = routes::session::router::ApiDoc, tags = ["Session"]),
            (path = "/users", api = routes::user::router::ApiDoc, tags = ["User"]),
        )
    )]
    struct ApiDoc;

    // Init router
    println!("Registering routes...");
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
        .nest("/auth", routes::auth::router::init())
        .nest("/department", routes::department::router::init())
        .nest("/groups", routes::group::router::init())
        .nest("/permissions", routes::permission::router::init())
        .nest("/roles", routes::role::router::init())
        .nest("/session", routes::session::router::init())
        .nest("/users", routes::user::router::init())
        .layer(Extension(db_conn));

    let server_port = setting.server_port;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", server_port))
        .await
        .unwrap();

    // Print info
    println!("{}", Info);

    // Start server
    axum::serve(listener, router)
        .await
        .map_err(|err| anyhow::anyhow!(err))?;

    Ok(())
}
