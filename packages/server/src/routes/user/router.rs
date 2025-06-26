use app::services::user::{UserService, create_user::CreateUserParams, query_user::UserOrderField};
use axum::Json;
use shared::enums::OperationPermission;
use utoipa::OpenApi;

use crate::{
    dto::PageableQueryDto,
    extractors::{app_service::AppService, helper::Helper, session::Session},
    init_router,
    response::{ApiResponse, PaginatedData, ResponseJson, ResponseJsonNull},
    result::ServerResult,
    routes::user::dto::DeleteUsersRequestDto,
};

use super::dto::{CreateUserDto, UserDto, UserFilterDto};

#[derive(OpenApi)]
#[openapi(paths(create_user, query_users_by_page, delete_users))]
pub(crate) struct ApiDoc;
init_router!(create_user, query_users_by_page, delete_users);

/// Query users by page
#[utoipa::path(
    operation_id = "queryUsersByPage",
    post,
    path = "/queryUsersByPage",
    request_body = PageableQueryDto<UserFilterDto, UserOrderField>,
    responses(
        (status = OK, description = "ok", body = ResponseJson<PaginatedData<UserDto>>)
    )
)]
#[axum::debug_handler]
pub async fn query_users_by_page(
    session: Session,
    user_service: AppService<UserService>,
    Json(params): Json<PageableQueryDto<UserFilterDto, UserOrderField>>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::QueryUsers)?;

    let (users, total) = user_service.query_users_by_page(params.into()).await?;
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
    session: Session,
    util: Helper,
    user_service: AppService<UserService>,
    Json(params): Json<CreateUserDto>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::CreateUser)?;
    let password = util.decrypt_rsa(&params.password)?;

    let user_id = user_service
        .create_user(CreateUserParams {
            account: params.account,
            password,
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
    session: Session,
    user_service: AppService<UserService>,
    Json(params): Json<DeleteUsersRequestDto>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::DeleteUser)?;

    user_service.delete_users(params.into()).await?;

    Ok(ApiResponse::null())
}
