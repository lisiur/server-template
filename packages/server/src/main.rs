use app::App;
use axum::{Extension, Router, routing::get};
use info::Info;
use result::ServerResult;
use rsa::{RsaPrivateKey, RsaPublicKey, pkcs1::DecodeRsaPrivateKey, pkcs8::DecodePublicKey};
use sea_orm::{Database, DatabaseConnection};
use settings::Settings;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;
use utoipa::{OpenApi, openapi::Server};
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

    // Init secret key
    println!("Loading private key...");
    let priv_key_pem = tokio::fs::read_to_string(&setting.private_key)
        .await
        .expect("Error: Failed to read private key");
    let priv_key =
        RsaPrivateKey::from_pkcs1_pem(&priv_key_pem).expect("Error: Invalid private key");

    println!("Loading public key...");
    let pub_key_pem = tokio::fs::read_to_string(&setting.public_key)
        .await
        .expect("Error: Failed to read public key");
    let pub_key =
        RsaPublicKey::from_public_key_pem(&pub_key_pem).expect("Error: Invalid public key");

    // Init uploader folder
    println!("Init upload directory");
    let upload_dir = setting.upload_dir.as_path();
    let upload_dir = if upload_dir.is_absolute() {
        upload_dir.to_path_buf()
    } else {
        std::env::current_dir()?.join(upload_dir)
    };
    if !upload_dir.exists() {
        std::fs::create_dir_all(&upload_dir)?;
    }

    // Connect database
    println!("Connecting database...");
    let db_conn: DatabaseConnection = Database::connect(&setting.database_url).await?;

    // Init app
    println!("Apply migrations...");
    let app = App::init(db_conn.clone(), upload_dir.to_path_buf()).await?;

    #[derive(OpenApi)]
    #[openapi(
        info(description = "OpenApi Docs"),
        nest(
            (path = "/system", api = routes::system::router::ApiDoc, tags = ["System"]),
            (path = "/auth", api = routes::auth::router::ApiDoc, tags = ["Auth"]),
            (path = "/session", api = routes::session::router::ApiDoc, tags = ["Session"]),
            (path = "/users", api = routes::user::router::ApiDoc, tags = ["User"]),
            (path = "/department", api = routes::department::router::ApiDoc, tags = ["Department"]),
            (path = "/userGroups", api = routes::user_group::router::ApiDoc, tags = ["UserGroup"]),
            (path = "/roles", api = routes::role::router::ApiDoc, tags = ["Role"]),
            (path = "/permissions", api = routes::permission::router::ApiDoc, tags = ["Permission"]),
            (path = "/uploads", api = routes::upload::router::ApiDoc, tags = ["Upload"]),
        ),
    )]
    struct ApiDoc;

    let mut openapi = ApiDoc::openapi();
    openapi.servers = Some(vec![Server::new("http://localhost:4000/api")]);
    let openapi_json = openapi.to_json().unwrap();

    // Init router
    println!("Registering routes...");
    let router = Router::new()
        .nest(
            "/api",
            Router::new()
                .route(
                    "/health",
                    get(|| async { (axum::http::StatusCode::OK, "OK") }),
                )
                .route("/openapi.json", get(|| async { axum::Json(openapi_json) }))
                .merge(Scalar::with_url("/docs", openapi.clone()))
                .nest("/auth", routes::auth::router::init())
                .nest("/department", routes::department::router::init())
                .nest("/groups", routes::user_group::router::init())
                .nest("/permissions", routes::permission::router::init())
                .nest("/roles", routes::role::router::init())
                .nest("/session", routes::session::router::init())
                .nest("/system", routes::system::router::init())
                .nest("/users", routes::user::router::init())
                .nest("/uploads", routes::upload::router::init()),
        )
        .layer(Extension(app))
        .layer(Extension(db_conn))
        .layer(Extension(priv_key))
        .layer(Extension(pub_key))
        .layer(TraceLayer::new_for_http());

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
