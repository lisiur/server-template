use app::{services::permission::PermissionService, utils::query::PaginatedQuery};
use axum::{
    Json, Router,
    extract::{Query, State},
    routing::{delete, get, post},
};
use utoipa::OpenApi;
use uuid::Uuid;

use crate::{
    dto::PaginatedQueryDto,
    permission::dto::{
        CreatePermissionDto, DeletePermissionsRequestDto, FilterPermissionsDto, PermissionDto,
    },
    rest::{PaginatedData, RestResponse, RestResponseJson, RestResponseJsonNull},
    result::ServerResult,
    state::AppState,
};

#[derive(OpenApi)]
#[openapi(paths(create_permission, query_permissions_by_page))]
pub(crate) struct ApiDoc;

pub(crate) fn init() -> Router<AppState> {
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
    State(state): State<AppState>,
    Query(query): Query<PaginatedQuery<FilterPermissionsDto>>,
) -> ServerResult<RestResponseJson<PaginatedData<PermissionDto>>> {
    let permission_service = PermissionService::new(state.db_conn);

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
    State(state): State<AppState>,
    Json(params): Json<CreatePermissionDto>,
) -> ServerResult<RestResponseJson<Uuid>> {
    let permission_service = PermissionService::new(state.db_conn);

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
    State(state): State<AppState>,
    Json(params): Json<DeletePermissionsRequestDto>,
) -> ServerResult<RestResponseJsonNull> {
    let permission_service = PermissionService::new(state.db_conn);

    permission_service.delete_permissions(params.into()).await?;

    Ok(RestResponse::null())
}
