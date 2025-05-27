use app::user::{create_user::CreateUserParams, UserService};
use axum::{
    extract::State, routing::{get, post}, Json, Router
};
use utoipa::OpenApi;
use uuid::Uuid;

use crate::{
    rest::{RestResponse, RestResponseJson},
    result::ServerResult,
    state::AppState,
};

use super::dto::CreateUserDto;

#[derive(OpenApi)]
#[openapi(paths(list_all_users, create_user))]
pub(crate) struct ApiDoc;

pub(crate) fn init() -> Router<AppState> {
    Router::new()
        .route("/", get(list_all_users))
        .route("/", post(create_user))
}

#[utoipa::path(
    operation_id = "listAllUsers",
    description = "List all users",
    get,
    path = "",
    responses(
        (status = OK, description = "ok", body = str)
    )
)]
// List all users
pub async fn list_all_users() -> ServerResult<RestResponseJson<String>> {
    RestResponse::json("".to_string()).into()
}

#[utoipa::path(
    operation_id = "createUser",
    description = "Create user",
    post,
    path = "",
    request_body = CreateUserDto,
    responses(
        (status = OK, description = "ok", body = ())
    )
)]
/// Create user
#[axum::debug_handler]
pub async fn create_user(
    State(state): State<AppState>,
    Json(params): Json<CreateUserDto>,
) -> ServerResult<RestResponseJson<Uuid>> {
    let user_service = UserService::new(state.db_conn);

    let user_id = user_service
        .create_user(CreateUserParams {
            account: params.account,
            password: params.password,
            ..Default::default()
        })
        .await?;

    Ok(RestResponse::json(user_id))
}
