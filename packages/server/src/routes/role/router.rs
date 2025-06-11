use app::{services::role::RoleService, utils::query::PaginatedQuery};
use axum::{
    Extension, Json, Router,
    extract::{Query, State},
    routing::{delete, get, patch, post},
};
use sea_orm::DatabaseConnection;
use utoipa::OpenApi;
use uuid::Uuid;

use crate::{
    dto::PaginatedQueryDto,
    rest::{Null, PaginatedData, RestResponse, RestResponseJson, RestResponseJsonNull},
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
        (status = OK, description = "ok", body = RestResponseJson<PaginatedData<RoleDto>>)
    )
)]
pub async fn query_roles_by_page(
    Extension(conn): Extension<DatabaseConnection>,
    Query(query): Query<PaginatedQuery<RoleFilterDto>>,
) -> ServerResult<RestResponseJson<PaginatedData<RoleDto>>> {
    let role_service = RoleService::new(conn);

    let (records, total) = role_service.query_roles_by_page(query).await?;
    let records = records.into_iter().map(RoleDto::from).collect::<Vec<_>>();

    Ok(RestResponse::json(PaginatedData { records, total }))
}

/// Create role
#[utoipa::path(
    operation_id = "createRole",
    description = "Create role",
    post,
    path = "/createRole",
    request_body = CreateRoleRequestDto,
    responses(
        (status = OK, description = "ok", body = RestResponseJson<Uuid>)
    )
)]
pub async fn create_role(
    Extension(conn): Extension<DatabaseConnection>,
    Json(params): Json<CreateRoleRequestDto>,
) -> ServerResult<RestResponseJson<Uuid>> {
    let role_service = RoleService::new(conn);

    let id = role_service.create_role(params.into()).await?;

    Ok(RestResponse::json(id))
}

/// Delete roles
#[utoipa::path(
    operation_id = "deleteRoles",
    description = "Delete roles",
    delete,
    path = "/deleteRoles",
    request_body = DeleteUsersRequestDto,
    responses(
        (status = OK, description = "ok", body = RestResponseJsonNull)
    )
)]
pub async fn delete_roles(
    Extension(conn): Extension<DatabaseConnection>,
    Json(params): Json<DeleteRolesRequestDto>,
) -> ServerResult<RestResponseJsonNull> {
    let role_service = RoleService::new(conn);

    role_service.delete_roles(params.into()).await?;

    Ok(RestResponse::null())
}

/// Update role
#[utoipa::path(
    operation_id = "updateRole",
    description = "Update role",
    patch,
    path = "/updateRole",
    request_body = UpdateRoleRequestDto,
    responses(
        (status = OK, description = "ok", body = RestResponseJson<Null>)
    )
)]
pub async fn update_role(
    Extension(conn): Extension<DatabaseConnection>,
    Json(params): Json<UpdateRoleRequestDto>,
) -> ServerResult<RestResponseJson<Null>> {
    let role_service = RoleService::new(conn);
    role_service.update_role(params.into()).await?;
    Ok(RestResponse::null())
}
