use app::{
    services::user::{UserService, create_user::CreateUserParams},
    utils::query::SelectQuery,
};
use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use utoipa::OpenApi;
use uuid::Uuid;

use crate::{
    rest::{PaginatedData, RestResponse, RestResponseJson},
    result::ServerResult,
    state::AppState,
};

use super::dto::{CreateUserDto, UserDto};

#[derive(OpenApi)]
#[openapi(paths(list_all_users, create_user, query_users_by_page))]
pub(crate) struct ApiDoc;

pub(crate) fn init() -> Router<AppState> {
    Router::new()
        .route("/", get(list_all_users))
        .route("/", post(create_user))
        .route("/page", get(query_users_by_page))
}

#[utoipa::path(
    operation_id = "listAllUsers",
    description = "List all users",
    get,
    path = "",
    responses(
        (status = OK, description = "ok", body = RestResponseJson<Vec<UserDto>>)
    )
)]
/// List all users
pub async fn list_all_users(
    State(state): State<AppState>,
) -> ServerResult<RestResponseJson<Vec<UserDto>>> {
    let user_service = UserService::new(state.db_conn);

    let users = user_service.query_users_list().await?;
    let users = users.into_iter().map(UserDto::from).collect();

    Ok(RestResponse::json(users))
}

#[utoipa::path(
    operation_id = "queryUsersByPage",
    description = "Query users by page",
    get,
    path = "/page",
    responses(
        (status = OK, description = "ok", body = RestResponseJson<PaginatedData<UserDto>>)
    )
)]
/// Query users by page
pub async fn query_users_by_page(
    State(state): State<AppState>,
    Json(params): Json<SelectQuery>,
) -> ServerResult<RestResponseJson<PaginatedData<UserDto>>> {
    let user_service = UserService::new(state.db_conn);

    let (users, total) = user_service.query_users_by_page(params).await?;
    let data = users.into_iter().map(UserDto::from).collect::<Vec<_>>();

    Ok(RestResponse::json(PaginatedData { data, total }))
}

#[utoipa::path(
    operation_id = "createUser",
    description = "Create user",
    post,
    path = "",
    request_body = CreateUserDto,
    responses(
        (status = OK, description = "ok", body = RestResponseJson<Uuid>)
    )
)]
/// Create user
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
