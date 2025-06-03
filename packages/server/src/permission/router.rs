use app::{services::permission::PermissionService, utils::query::SelectQuery};
use axum::{
    Json, Router,
    extract::{Query, State},
    routing::{get, post},
};
use utoipa::OpenApi;
use uuid::Uuid;

use crate::{
    dto::{PaginatedQuery, PaginatedQueryDto},
    rest::{PaginatedData, RestResponse, RestResponseJson},
    result::ServerResult,
    state::AppState,
};

use super::dto::{CreatePermissionDto, PermissionDto, PermissionFilterDto};

#[derive(OpenApi)]
#[openapi(paths(create_permission, query_permissions_by_page))]
pub(crate) struct ApiDoc;

pub(crate) fn init() -> Router<AppState> {
    Router::new()
        .route("/", post(create_permission))
        .route("/page", get(query_permissions_by_page))
}

#[utoipa::path(
    operation_id = "queryPermissionsByPage",
    description = "Query permissions by page",
    get,
    path = "/page",
    params(PaginatedQueryDto, PermissionFilterDto),
    responses(
        (status = OK, description = "ok", body = RestResponseJson<PaginatedData<PermissionDto>>)
    )
)]
/// Query permissions by page
pub async fn query_permissions_by_page(
    State(state): State<AppState>,
    Query(query): Query<PaginatedQuery<PermissionFilterDto>>,
) -> ServerResult<RestResponseJson<PaginatedData<PermissionDto>>> {
    let permission_service = PermissionService::new(state.db_conn);

    let select_query = SelectQuery::default().with_cursor(query.cursor());

    let (records, total) = permission_service
        .query_permissions_by_page(select_query)
        .await?;

    let records = records
        .into_iter()
        .map(PermissionDto::from)
        .collect::<Vec<_>>();

    Ok(RestResponse::json(PaginatedData { records, total }))
}

#[utoipa::path(
    operation_id = "createPermission",
    description = "Create permission",
    post,
    path = "",
    request_body = CreatePermissionDto,
    responses(
        (status = OK, description = "ok", body = RestResponseJson<Uuid>)
    )
)]
/// Create permission
pub async fn create_permission(
    State(state): State<AppState>,
    Json(params): Json<CreatePermissionDto>,
) -> ServerResult<RestResponseJson<Uuid>> {
    let permission_service = PermissionService::new(state.db_conn);

    let user_id = permission_service.create_permission(params.into()).await?;

    Ok(RestResponse::json(user_id))
}
