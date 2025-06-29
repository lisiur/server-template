use app::services::role::RoleService;
use axum::Json;
use shared::enums::OperationPermission;
use utoipa::OpenApi;

use crate::{
    dto::PageableQueryDto,
    extractors::{app_service::AppService, session::Session},
    init_router,
    response::{ApiResponse, Null, PaginatedData, ResponseJson, ResponseJsonNull},
    result::ServerResult,
    routes::{
        role::dto::{DeleteRolesRequestDto, UpdateRoleRequestDto},
        user::dto::DeleteUsersRequestDto,
    },
};

use super::dto::{CreateRoleRequestDto, RoleDto, RoleFilterDto};

#[derive(OpenApi)]
#[openapi(paths(query_roles_by_page, create_role, delete_roles, update_role))]
pub(crate) struct ApiDoc;
init_router!(query_roles_by_page, create_role, delete_roles, update_role);

/// Query roles by page
#[utoipa::path(
    operation_id = "queryRolesByPage",
    description = "Query roles by page",
    post,
    path = "/queryRolesByPage",
    request_body = PageableQueryDto<RoleFilterDto>,
    responses(
        (status = OK, description = "ok", body = ResponseJson<PaginatedData<RoleDto>>)
    )
)]
pub async fn query_roles_by_page(
    session: Session,
    role_service: AppService<RoleService>,
    Json(params): Json<PageableQueryDto<RoleFilterDto>>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::QueryRoles)?;

    let (records, total) = role_service.query_roles_by_page(params.into()).await?;
    let records = records.into_iter().map(RoleDto::from).collect::<Vec<_>>();

    Ok(ApiResponse::json(PaginatedData { records, total }))
}

/// Create role
#[utoipa::path(
    operation_id = "createRole",
    description = "Create role",
    post,
    path = "/createRole",
    request_body = CreateRoleRequestDto,
    responses(
        (status = OK, description = "ok", body = ResponseJson<Uuid>)
    )
)]
pub async fn create_role(
    session: Session,
    role_service: AppService<RoleService>,
    Json(params): Json<CreateRoleRequestDto>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::CreateRole)?;

    let id = role_service.create_role(params.into()).await?;

    Ok(ApiResponse::json(id))
}

/// Delete roles
#[utoipa::path(
    operation_id = "deleteRoles",
    description = "Delete roles",
    delete,
    path = "/deleteRoles",
    request_body = DeleteUsersRequestDto,
    responses(
        (status = OK, description = "ok", body = ResponseJsonNull)
    )
)]
pub async fn delete_roles(
    session: Session,
    role_service: AppService<RoleService>,
    Json(params): Json<DeleteRolesRequestDto>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::DeleteRole)?;

    role_service.delete_roles(params.into()).await?;

    Ok(ApiResponse::null())
}

/// Update role
#[utoipa::path(
    operation_id = "updateRole",
    description = "Update role",
    patch,
    path = "/updateRole",
    request_body = UpdateRoleRequestDto,
    responses(
        (status = OK, description = "ok", body = ResponseJson<Null>)
    )
)]
pub async fn update_role(
    session: Session,
    role_service: AppService<RoleService>,
    Json(params): Json<UpdateRoleRequestDto>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::UpdateRole)?;

    role_service.update_role(params.into()).await?;
    Ok(ApiResponse::null())
}
