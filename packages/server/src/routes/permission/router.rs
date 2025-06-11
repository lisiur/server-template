use app::{services::permission::PermissionService, utils::query::PaginatedQuery};
use axum::{
    Extension, Json, Router,
    extract::{Query, State},
    routing::{delete, get, post},
};
use sea_orm::DatabaseConnection;
use utoipa::OpenApi;
use uuid::Uuid;

use crate::{
    dto::PaginatedQueryDto,
    rest::{PaginatedData, RestResponse, RestResponseJson, RestResponseJsonNull},
    result::ServerResult,
    routes::permission::dto::{
        CreatePermissionDto, DeletePermissionsRequestDto, FilterPermissionsDto, PermissionDto,
    },
};

#[derive(OpenApi)]
#[openapi(paths(create_permission, query_permissions_by_page))]
pub(crate) struct ApiDoc;

pub(crate) fn init() -> Router {
    Router::new()
        .route("/createPermission", post(create_permission))
        .route("/queryPermissionsByPage", get(query_permissions_by_page))
        .route("/deletePermissions", delete(delete_permissions))
}

/// Query permissions by page
#[utoipa::path(
    operation_id = "queryPermissionsByPage",
    description = "Query permissions by page",
    get,
    path = "/queryPermissionsByPage",
    params(PaginatedQueryDto, FilterPermissionsDto),
    responses(
        (status = OK, description = "ok", body = RestResponseJson<PaginatedData<PermissionDto>>)
    )
)]
pub async fn query_permissions_by_page(
    Extension(conn): Extension<DatabaseConnection>,
    Query(query): Query<PaginatedQuery<FilterPermissionsDto>>,
) -> ServerResult<RestResponseJson<PaginatedData<PermissionDto>>> {
    let permission_service = PermissionService::new(conn);

    let (records, total) = permission_service.query_permissions_by_page(query).await?;

    let records = records
        .into_iter()
        .map(PermissionDto::from)
        .collect::<Vec<_>>();

    Ok(RestResponse::json(PaginatedData { records, total }))
}

/// Create permission
#[utoipa::path(
    operation_id = "createPermission",
    description = "Create permission",
    post,
    path = "/createPermission",
    request_body = CreatePermissionDto,
    responses(
        (status = OK, description = "ok", body = RestResponseJson<Uuid>)
    )
)]
pub async fn create_permission(
    Extension(conn): Extension<DatabaseConnection>,
    Json(params): Json<CreatePermissionDto>,
) -> ServerResult<RestResponseJson<Uuid>> {
    let permission_service = PermissionService::new(conn);

    let user_id = permission_service.create_permission(params.into()).await?;

    Ok(RestResponse::json(user_id))
}

/// Delete permission
#[utoipa::path(
    operation_id = "deletePermissions",
    description = "Delete permissions",
    delete,
    path = "/deletePermissions",
    request_body = DeletePermissionsRequestDto,
    responses(
        (status = OK, description = "ok", body = RestResponseJsonNull)
    )
)]
pub async fn delete_permissions(
    Extension(conn): Extension<DatabaseConnection>,
    Json(params): Json<DeletePermissionsRequestDto>,
) -> ServerResult<RestResponseJsonNull> {
    let permission_service = PermissionService::new(conn);

    permission_service.delete_permissions(params.into()).await?;

    Ok(RestResponse::null())
}
