use app::{
    services::user::{UserService, create_user::CreateUserParams},
    utils::query::PaginatedQuery,
};
use axum::{Extension, Json, extract::Query};
use sea_orm::DatabaseConnection;
use shared::enums::OperationPermission;
use utoipa::OpenApi;

use crate::{
    dto::PaginatedQueryDto,
    extractors::auth_session::AuthSession,
    init_router,
    response::{ApiResponse, PaginatedData, ResponseJson, ResponseJsonNull},
    result::ServerResult,
    routes::user::dto::DeleteUsersRequestDto,
};

use super::dto::{CreateUserDto, FilterUserDto, UserDto};

#[derive(OpenApi)]
#[openapi(paths(query_users, create_user, query_users_by_page, delete_users))]
pub(crate) struct ApiDoc;
init_router!(query_users, create_user, query_users_by_page, delete_users);

/// Query users
#[utoipa::path(
    operation_id = "queryUsers",
    get,
    path = "/queryUsers",
    responses(
        (status = OK, description = "ok", body = ResponseJson<Vec<UserDto>>)
    )
)]
pub async fn query_users(
    session: AuthSession,
    Extension(conn): Extension<DatabaseConnection>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::QueryUsers)?;

    let user_service = UserService::new(conn);

    let users = user_service.query_users_list().await?;
    let users = users.into_iter().map(UserDto::from).collect::<Vec<_>>();

    Ok(ApiResponse::json(users))
}

/// Query users by page
#[utoipa::path(
    operation_id = "queryUsersByPage",
    get,
    path = "/queryUsersByPage",
    params(PaginatedQueryDto, FilterUserDto),
    responses(
        (status = OK, description = "ok", body = ResponseJson<PaginatedData<UserDto>>)
    )
)]
pub async fn query_users_by_page(
    session: AuthSession,
    Extension(conn): Extension<DatabaseConnection>,
    Query(query): Query<PaginatedQuery<FilterUserDto>>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::QueryUsers)?;

    let user_service = UserService::new(conn);

    let (users, total) = user_service.query_users_by_page(query).await?;
    let records = users.into_iter().map(UserDto::from).collect::<Vec<_>>();

    Ok(ApiResponse::json(PaginatedData { records, total }))
}

/// Create user
#[utoipa::path(
    operation_id = "createUser",
    post,
    path = "/createUser",
    request_body = CreateUserDto,
    responses(
        (status = OK, description = "ok", body = ResponseJson<Uuid>)
    )
)]
pub async fn create_user(
    session: AuthSession,
    Extension(conn): Extension<DatabaseConnection>,
    Json(params): Json<CreateUserDto>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::CreateUser)?;

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
    delete,
    path = "/deleteUsers",
    request_body = DeleteUsersRequestDto,
    responses(
        (status = OK, description = "ok", body = ResponseJsonNull)
    )
)]
pub async fn delete_users(
    session: AuthSession,
    Extension(conn): Extension<DatabaseConnection>,
    Json(params): Json<DeleteUsersRequestDto>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::DeleteUser)?;

    let user_service = UserService::new(conn);

    user_service.delete_users(params.into()).await?;

    Ok(ApiResponse::null())
}
