use app::user::UserService;
use axum::{
    Router,
    extract::State,
    routing::{get, post},
};
use utoipa::OpenApi;
use uuid::Uuid;

use crate::{
    rest::{RestResponse, RestResponseJson},
    result::ServerResult,
    state::AppState,
};

#[derive(OpenApi)]
#[openapi(paths(all_users, create_user))]
pub(crate) struct ApiDoc;

pub(crate) fn init() -> Router<AppState> {
    Router::new()
        .route("/", get(all_users))
        .route("/create", post(create_user))
}

#[utoipa::path(
    get,
    path = "",
    responses(
        (status = OK, description = "ok", body = str)
    )
)]
pub async fn all_users() -> ServerResult<RestResponseJson<String>> {
    RestResponse::json("".to_string()).into()
}

#[utoipa::path(
    post,
    path = "/create",
    responses(
        (status = OK, description = "ok", body = ())
    )
)]
pub async fn create_user(State(state): State<AppState>) -> ServerResult<RestResponseJson<Uuid>> {
    let user_service = UserService::new(state.db_conn);

    let user_id = user_service.create_user().await?;

    Ok(RestResponse::json(user_id))
}
