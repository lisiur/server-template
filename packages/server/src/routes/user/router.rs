use app::{
    services::user::{UserService, create_user::CreateUserParams},
    utils::query::PaginatedQuery,
};
use axum::{
    Extension, Json, Router,
    extract::Query,
    routing::{delete, get, post},
};
use sea_orm::DatabaseConnection;
use utoipa::OpenApi;

use crate::{
    dto::PaginatedQueryDto,
    response::{ApiResponse, PaginatedData, ResponseJson, ResponseJsonNull},
    result::ServerResult,
    routes::user::dto::DeleteUsersRequestDto,
};

use super::dto::{CreateUserDto, FilterUserDto, UserDto};

#[derive(OpenApi)]
#[openapi(paths(query_users, create_user, query_users_by_page))]
pub(crate) struct ApiDoc;

pub(crate) fn init() -> Router {
    Router::new()
        .route("/listUsers", get(query_users))
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
        (status = OK, description = "ok", body = ResponseJson<Vec<UserDto>>)
    )
)]
/// Query users
pub async fn query_users(
    Extension(conn): Extension<DatabaseConnection>,
) -> ServerResult<ApiResponse> {
    let user_service = UserService::new(conn);

    let users = user_service.query_users_list().await?;
    let users = users.into_iter().map(UserDto::from).collect::<Vec<_>>();

    Ok(ApiResponse::json(users))
}

#[utoipa::path(
    operation_id = "queryUsersByPage",
    description = "Query users by page",
    get,
    path = "/queryUsersByPage",
    params(PaginatedQueryDto, FilterUserDto),
    responses(
        (status = OK, description = "ok", body = ResponseJson<PaginatedData<UserDto>>)
    )
)]
/// Query users by page
pub async fn query_users_by_page(
    Extension(conn): Extension<DatabaseConnection>,
    Query(query): Query<PaginatedQuery<FilterUserDto>>,
) -> ServerResult<ApiResponse> {
    let user_service = UserService::new(conn);

    let (users, total) = user_service.query_users_by_page(query).await?;
    let records = users.into_iter().map(UserDto::from).collect::<Vec<_>>();

    Ok(ApiResponse::json(PaginatedData { records, total }))
}

#[utoipa::path(
    operation_id = "createUser",
    description = "Create user",
    post,
    path = "/createUser",
    request_body = CreateUserDto,
    responses(
        (status = OK, description = "ok", body = ResponseJson<Uuid>)
    )
)]
/// Create user
pub async fn create_user(
    Extension(conn): Extension<DatabaseConnection>,
    Json(params): Json<CreateUserDto>,
) -> ServerResult<ApiResponse> {
    let user_service = UserService::new(conn);

    let user_id = user_service
        .create_user(CreateUserParams {
            account: params.account,
            password: params.password,
            ..Default::default()
        })
        .await?;

    Ok(ApiResponse::json(user_id))
}

/// Delete users
#[utoipa::path(
    operation_id = "deleteUsers",
    description = "Delete users",
    delete,
    path = "/deleteUsers",
    request_body = DeleteUsersRequestDto,
    responses(
        (status = OK, description = "ok", body = ResponseJsonNull)
    )
)]
pub async fn delete_users(
    Extension(conn): Extension<DatabaseConnection>,
    Json(params): Json<DeleteUsersRequestDto>,
) -> ServerResult<ApiResponse> {
    let user_service = UserService::new(conn);

    user_service.delete_users(params.into()).await?;

    Ok(ApiResponse::null())
}
