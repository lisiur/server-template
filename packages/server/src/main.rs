use app::App;
use axum::{Router, routing::get};
use info::Info;
use result::ServerResult;
use sea_orm::{Database, DatabaseConnection};
use settings::Settings;
use state::AppState;
use tower_cookies::CookieManagerLayer;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

mod auth;
mod dto;
mod error;
mod info;
mod permission;
mod rest;
mod result;
mod session;
mod settings;
mod state;
mod user;

#[tokio::main]
async fn main() -> ServerResult<()> {
    // Load .env
    dotenvy::dotenv().ok();

    // Init logger
    tracing_subscriber::fmt::init();

    // Init settings
    let setting = Settings::init()?;

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
            (path = "/permissions", api = permission::router::ApiDoc, tags = ["Permission"]),
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
        .nest("/permissions", permission::router::init())
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
