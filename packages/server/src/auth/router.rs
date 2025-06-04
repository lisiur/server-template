use app::services::auth::AuthService;
use axum::{
    Json, Router,
    extract::{Query, State},
    routing::{get, post},
};
use utoipa::OpenApi;

use crate::{
    auth::dto::{GroupChainPermissionsDto, QueryGroupChainPermissionsDto, QueryUserPermissionsDto},
    permission::dto::PermissionDto,
    rest::{Null, RestResponse, RestResponseJson},
    result::ServerResult,
    state::AppState,
};

use super::dto::{AssignUserPermissionsDto, GroupTreePermissionsDto, QueryGroupTreePermissionsDto};

#[derive(OpenApi)]
#[openapi(paths(
    assign_user_permissions,
    query_user_permissions,
    query_group_tree_permissions,
    query_group_chain_permissions,
))]
pub(crate) struct ApiDoc;

pub(crate) fn init() -> Router<AppState> {
    Router::new()
        .route("/assignUserPermissions", post(assign_user_permissions))
        .route("/queryUserPermissions", get(query_user_permissions))
        .route(
            "/queryGroupTreePermissions",
            get(query_group_tree_permissions),
        )
        .route(
            "/queryGroupChainPermissions",
            get(query_group_chain_permissions),
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
/// Assign user permissions
pub async fn assign_user_permissions(
    State(state): State<AppState>,
    Json(params): Json<AssignUserPermissionsDto>,
) -> ServerResult<RestResponseJson<Null>> {
    let auth_service = AuthService::new(state.db_conn);

    auth_service.assign_user_permissions(params.into()).await?;

    Ok(RestResponse::null())
}

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
/// Query user permission
pub async fn query_user_permissions(
    State(state): State<AppState>,
    Query(query): Query<QueryUserPermissionsDto>,
) -> ServerResult<RestResponseJson<Vec<PermissionDto>>> {
    let auth_service = AuthService::new(state.db_conn);

    let res = auth_service.query_user_permissions(query.user_id).await?;
    let res = res.into_iter().map(PermissionDto::from).collect();

    Ok(RestResponse::json(res))
}

#[utoipa::path(
    operation_id = "queryGroupTreePermissions",
    description = "Query group tree permissions",
    get,
    path = "/queryGroupTreePermissions",
    params(QueryGroupTreePermissionsDto),
    responses(
        (status = OK, description = "ok", body = RestResponseJson<GroupTreePermissionsDto>)
    )
)]
/// Query group tree permissions
pub async fn query_group_tree_permissions(
    State(state): State<AppState>,
    Query(query): Query<QueryGroupTreePermissionsDto>,
) -> ServerResult<RestResponseJson<GroupTreePermissionsDto>> {
    let auth_service = AuthService::new(state.db_conn);

    let res = auth_service
        .query_group_tree_permissions(query.group_id)
        .await?;
    let res = GroupTreePermissionsDto(res.0);

    Ok(RestResponse::json(res))
}

#[utoipa::path(
    operation_id = "queryGroupChainPermissions",
    description = "Query group chain permissions",
    get,
    path = "/queryGroupChainPermissions",
    params(QueryGroupTreePermissionsDto),
    responses(
        (status = OK, description = "ok", body = RestResponseJson<GroupTreePermissionsDto>)
    )
)]
/// Query group chain permissions
pub async fn query_group_chain_permissions(
    State(state): State<AppState>,
    Query(query): Query<QueryGroupChainPermissionsDto>,
) -> ServerResult<RestResponseJson<GroupChainPermissionsDto>> {
    let auth_service = AuthService::new(state.db_conn);

    let res = auth_service
        .query_group_chain_permissions(query.group_id)
        .await?;

    Ok(RestResponse::json(GroupChainPermissionsDto(res)))
}
