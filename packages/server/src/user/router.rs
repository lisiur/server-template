use app::{
    services::user::{UserService, create_user::CreateUserParams},
    utils::query::PaginatedQuery,
};
use axum::{
    Json, Router,
    extract::{Query, State},
    routing::{delete, get, post},
};
use utoipa::OpenApi;
use uuid::Uuid;

use crate::{
    dto::PaginatedQueryDto,
    rest::{PaginatedData, RestResponse, RestResponseJson, RestResponseJsonNull},
    result::ServerResult,
    state::AppState,
    user::dto::DeleteUsersRequestDto,
};

use super::dto::{CreateUserDto, FilterUserDto, UserDto};

#[derive(OpenApi)]
#[openapi(paths(list_users, create_user, query_users_by_page))]
pub(crate) struct ApiDoc;

pub(crate) fn init() -> Router<AppState> {
    Router::new()
        .route("/listUsers", get(list_users))
        .route("/createUser", post(create_user))
        .route("/queryUsersByPage", get(query_users_by_page))
        .route("/deleteUsers", delete(delete_users))
}

#[utoipa::path(
    operation_id = "listUsers",
    description = "List users",
    get,
    path = "/listUsers",
    responses(
        (status = OK, description = "ok", body = RestResponseJson<Vec<UserDto>>)
    )
)]
/// List all users
pub async fn list_users(
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
    path = "/queryUsersByPage",
    params(PaginatedQueryDto, FilterUserDto),
    responses(
        (status = OK, description = "ok", body = RestResponseJson<PaginatedData<UserDto>>)
    )
)]
/// Query users by page
pub async fn query_users_by_page(
    State(state): State<AppState>,
    Query(query): Query<PaginatedQuery<FilterUserDto>>,
) -> ServerResult<RestResponseJson<PaginatedData<UserDto>>> {
    let user_service = UserService::new(state.db_conn);

    let (users, total) = user_service.query_users_by_page(query).await?;
    let records = users.into_iter().map(UserDto::from).collect::<Vec<_>>();

    Ok(RestResponse::json(PaginatedData { records, total }))
}

#[utoipa::path(
    operation_id = "createUser",
    description = "Create user",
    post,
    path = "/createUser",
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

/// Delete users
#[utoipa::path(
    operation_id = "deleteUsers",
    description = "Delete users",
    delete,
    path = "/deleteUsers",
    request_body = DeleteUsersRequestDto,
    responses(
        (status = OK, description = "ok", body = RestResponseJsonNull)
    )
)]
pub async fn delete_users(
    State(state): State<AppState>,
    Json(params): Json<DeleteUsersRequestDto>,
) -> ServerResult<RestResponseJsonNull> {
    let user_service = UserService::new(state.db_conn);

    user_service.delete_users(params.into()).await?;

    Ok(RestResponse::null())
}
