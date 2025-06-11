use app::App;
use axum::{Extension, Router, routing::get};
use info::Info;
use result::ServerResult;
use sea_orm::{Database, DatabaseConnection};
use settings::Settings;
use state::AppState;
use time::Duration;
use tokio::{signal, task::AbortHandle};
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::PostgresStore;
use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

mod dto;
mod error;
mod extractors;
mod info;
mod middlewares;
mod rest;
mod result;
mod routes;
mod settings;
mod state;

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

    // Session layer
    let session_store = PostgresStore::new(db_conn.get_postgres_connection_pool().clone());
    session_store.migrate().await?;
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(1)));

    // Init router
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
        .layer(session_layer)
        .layer(Extension(db_conn));

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

async fn shutdown_signal(deletion_task_abort_handle: AbortHandle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { deletion_task_abort_handle.abort() },
        _ = terminate => { deletion_task_abort_handle.abort() },
    }
}
