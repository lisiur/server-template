use app::{services::role::RoleService, utils::query::PaginatedQuery};
use axum::{
    Extension, Json, Router,
    extract::Query,
    routing::{delete, get, patch, post},
};
use sea_orm::DatabaseConnection;
use utoipa::OpenApi;

use crate::{
    dto::PaginatedQueryDto,
    response::{ApiResponse, Null, PaginatedData, ResponseJson, ResponseJsonNull},
    result::ServerResult,
    routes::role::dto::{DeleteRolesRequestDto, UpdateRoleRequestDto},
    routes::user::dto::DeleteUsersRequestDto,
};

use super::dto::{CreateRoleRequestDto, RoleDto, RoleFilterDto};

#[derive(OpenApi)]
#[openapi(paths(query_roles_by_page, create_role, delete_roles, update_role))]
pub(crate) struct ApiDoc;

pub(crate) fn init() -> Router {
    Router::new()
        .route("/queryRolesByPage", get(query_roles_by_page))
        .route("/createRole", post(create_role))
        .route("/deleteRoles", delete(delete_roles))
        .route("/updateRole", patch(update_role))
}

/// Query roles by page
#[utoipa::path(
    operation_id = "queryRolesByPage",
    description = "Query roles by page",
    get,
    path = "/queryRolesByPage",
    params(PaginatedQueryDto, RoleFilterDto),
    responses(
        (status = OK, description = "ok", body = ResponseJson<PaginatedData<RoleDto>>)
    )
)]
pub async fn query_roles_by_page(
    Extension(conn): Extension<DatabaseConnection>,
    Query(query): Query<PaginatedQuery<RoleFilterDto>>,
) -> ServerResult<ApiResponse> {
    let role_service = RoleService::new(conn);

    let (records, total) = role_service.query_roles_by_page(query).await?;
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
    Extension(conn): Extension<DatabaseConnection>,
    Json(params): Json<CreateRoleRequestDto>,
) -> ServerResult<ApiResponse> {
    let role_service = RoleService::new(conn);

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
    Extension(conn): Extension<DatabaseConnection>,
    Json(params): Json<DeleteRolesRequestDto>,
) -> ServerResult<ApiResponse> {
    let role_service = RoleService::new(conn);

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
    Extension(conn): Extension<DatabaseConnection>,
    Json(params): Json<UpdateRoleRequestDto>,
) -> ServerResult<ApiResponse> {
    let role_service = RoleService::new(conn);
    role_service.update_role(params.into()).await?;
    Ok(ApiResponse::null())
}
