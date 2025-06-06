use app::services::auth::AuthService;
use axum::{
    Json, Router,
    extract::{Query, State},
    routing::{get, post},
};
use utoipa::OpenApi;

use crate::{
    auth::dto::{QueryDepartmentPermissionsDto, QueryGroupPermissionsDto, QueryUserPermissionsDto},
    permission::dto::PermissionDto,
    rest::{Null, RestResponse, RestResponseJson},
    result::ServerResult,
    state::AppState,
};

use super::dto::{AssignUserPermissionsDto, GroupTreePermissionsDto};

#[derive(OpenApi)]
#[openapi(paths(
    assign_user_permissions,
    query_user_permissions,
    query_group_permissions,
    query_department_permissions,
))]
pub(crate) struct ApiDoc;

/// Assign user permissions
pub(crate) fn init() -> Router<AppState> {
    Router::new()
        .route("/assignUserPermissions", post(assign_user_permissions))
        .route("/queryUserPermissions", get(query_user_permissions))
        .route("/queryGroupPermissions", get(query_group_permissions))
        .route(
            "/queryDepartmentPermissions",
            get(query_department_permissions),
        )
}

#[utoipa::path(
    operation_id = "assignUserPermissions",
    description = "Assign user permissions",
    post,
    path = "/assignUserPermissions",
    request_body = AssignUserPermissionsDto,
    responses(
        (status = OK, description = "ok", body = RestResponseJson<Null>)
    )
)]
pub async fn assign_user_permissions(
    State(state): State<AppState>,
    Json(params): Json<AssignUserPermissionsDto>,
) -> ServerResult<RestResponseJson<Null>> {
    let auth_service = AuthService::new(state.db_conn);

    auth_service.assign_user_permissions(params.into()).await?;

    Ok(RestResponse::null())
}

/// Query user permission
#[utoipa::path(
    operation_id = "queryUserPermissions",
    description = "Query user permissions",
    get,
    path = "/queryUserPermissions",
    params(QueryUserPermissionsDto),
    responses(
        (status = OK, description = "ok", body = RestResponseJson<Vec<PermissionDto>>)
    )
)]
pub async fn query_user_permissions(
    State(state): State<AppState>,
    Query(query): Query<QueryUserPermissionsDto>,
) -> ServerResult<RestResponseJson<Vec<PermissionDto>>> {
    let auth_service = AuthService::new(state.db_conn);

    let res = auth_service.query_user_permissions(query.user_id).await?;
    let res = res.into_iter().map(PermissionDto::from).collect();

    Ok(RestResponse::json(res))
}

/// Query group permissions
#[utoipa::path(
    operation_id = "queryGroupPermissions",
    description = "Query group permissions",
    get,
    path = "/queryGroupPermissions",
    params(QueryGroupPermissionsDto),
    responses(
        (status = OK, description = "ok", body = RestResponseJson<GroupTreePermissionsDto>)
    )
)]
pub async fn query_group_permissions(
    State(state): State<AppState>,
    Query(query): Query<QueryGroupPermissionsDto>,
) -> ServerResult<RestResponseJson<Vec<PermissionDto>>> {
    let auth_service = AuthService::new(state.db_conn);

    let res = auth_service.query_group_permissions(query.group_id).await?;

    Ok(RestResponse::json(
        res.into_iter()
            .map(PermissionDto::from)
            .collect::<Vec<PermissionDto>>(),
    ))
}

/// Query department permissions
#[utoipa::path(
    operation_id = "queryDepartmentPermissions",
    description = "Query department permissions",
    get,
    path = "/queryDepartmentPermissions",
    params(QueryDepartmentPermissionsDto),
    responses(
        (status = OK, description = "ok", body = RestResponseJson<Vec<PermissionDto>>)
    )
)]
pub async fn query_department_permissions(
    State(state): State<AppState>,
    Query(query): Query<QueryDepartmentPermissionsDto>,
) -> ServerResult<RestResponseJson<Vec<PermissionDto>>> {
    let auth_service = AuthService::new(state.db_conn);

    let res = auth_service
        .query_department_permissions(query.department_id)
        .await?;

    Ok(RestResponse::json(
        res.into_iter()
            .map(PermissionDto::from)
            .collect::<Vec<PermissionDto>>(),
    ))
}
