use app::services::permission::PermissionService;
use axum::{Extension, Json};
use sea_orm::DatabaseConnection;
use shared::enums::OperationPermission;
use utoipa::OpenApi;

use crate::{
    dto::PageableQueryDto,
    extractors::session::Session,
    init_router,
    response::{ApiResponse, Null, PaginatedData, ResponseJson, ResponseJsonNull},
    result::ServerResult,
    routes::permission::dto::{
        CreatePermissionDto, DeletePermissionsRequestDto, FilterPermissionsDto, PermissionDto,
    },
};

use super::dto::UpdatePermissionRequestDto;

#[derive(OpenApi)]
#[openapi(paths(
    create_permission,
    query_permissions_by_page,
    delete_permissions,
    update_permission
))]
pub(crate) struct ApiDoc;
init_router!(
    create_permission,
    query_permissions_by_page,
    delete_permissions,
    update_permission
);

/// Query permissions by page
#[utoipa::path(
    operation_id = "queryPermissionsByPage",
    description = "Query permissions by page",
    get,
    path = "/queryPermissionsByPage",
    request_body = PageableQueryDto<FilterPermissionsDto>,
    responses(
        (status = OK, description = "ok", body = ResponseJson<PaginatedData<PermissionDto>>)
    )
)]
pub async fn query_permissions_by_page(
    session: Session,
    Extension(conn): Extension<DatabaseConnection>,
    Json(params): Json<PageableQueryDto<FilterPermissionsDto>>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::QueryPermissions)?;

    let permission_service = PermissionService::new(conn);

    let (records, total) = permission_service
        .query_permissions_by_page(params.into())
        .await?;

    let records = records
        .into_iter()
        .map(PermissionDto::from)
        .collect::<Vec<_>>();

    Ok(ApiResponse::json(PaginatedData { records, total }))
}

/// Create permission
#[utoipa::path(
    operation_id = "createPermission",
    description = "Create permission",
    post,
    path = "/createPermission",
    request_body = CreatePermissionDto,
    responses(
        (status = OK, description = "ok", body = ResponseJson<Uuid>)
    )
)]
pub async fn create_permission(
    session: Session,
    Extension(conn): Extension<DatabaseConnection>,
    Json(params): Json<CreatePermissionDto>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::CreatePermission)?;

    let permission_service = PermissionService::new(conn);

    let user_id = permission_service.create_permission(params.into()).await?;

    Ok(ApiResponse::json(user_id))
}

/// Delete permission
#[utoipa::path(
    operation_id = "deletePermissions",
    description = "Delete permissions",
    delete,
    path = "/deletePermissions",
    request_body = DeletePermissionsRequestDto,
    responses(
        (status = OK, description = "ok", body = ResponseJsonNull)
    )
)]
pub async fn delete_permissions(
    session: Session,
    Extension(conn): Extension<DatabaseConnection>,
    Json(params): Json<DeletePermissionsRequestDto>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::DeletePermission)?;

    let permission_service = PermissionService::new(conn);

    permission_service.delete_permissions(params.into()).await?;

    Ok(ApiResponse::null())
}

/// Update permission
#[utoipa::path(
    operation_id = "updatePermission",
    description = "Update permission",
    patch,
    path = "/updatePermission",
    request_body = UpdatePermissionRequestDto,
    responses(
        (status = OK, description = "ok", body = ResponseJson<Null>)
    )
)]
pub async fn update_permission(
    session: Session,
    Extension(conn): Extension<DatabaseConnection>,
    Json(params): Json<UpdatePermissionRequestDto>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::UpdateGroup)?;

    let group_service = PermissionService::new(conn);
    group_service.update_permission(params.into()).await?;
    Ok(ApiResponse::null())
}
