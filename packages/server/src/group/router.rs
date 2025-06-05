use app::{
    services::group::{GroupService, create_group::CreateGroupParams},
    utils::query::PaginatedQuery,
};
use axum::{
    Json, Router,
    extract::{Query, State},
    routing::{delete, get, patch, post},
};
use utoipa::OpenApi;

use crate::{
    dto::PaginatedQueryDto,
    group::dto::{CreateGroupResponseDto, DeleteGroupsRequestDto, UpdateGroupRequestDto},
    rest::{Null, PaginatedData, RestResponse, RestResponseJson},
    result::ServerResult,
    state::AppState,
};

use super::dto::{CreateGroupRequestDto, FilterGroupsDto, GroupDto};

#[derive(OpenApi)]
#[openapi(paths(create_group, query_groups_by_page, delete_groups, update_group))]
pub(crate) struct ApiDoc;

pub(crate) fn init() -> Router<AppState> {
    Router::new()
        .route("/createGroup", post(create_group))
        .route("/queryGroupsByPage", get(query_groups_by_page))
        .route("/deleteGroups", delete(delete_groups))
        .route("/updateGroup", patch(update_group))
}

/// Query groups by page
#[utoipa::path(
    operation_id = "queryGroupsByPage",
    description = "Query groups by page",
    get,
    path = "/queryGroupsByPage",
    params(PaginatedQueryDto, FilterGroupsDto),
    responses(
        (status = OK, description = "ok", body = RestResponseJson<PaginatedData<GroupDto>>)
    )
)]
pub async fn query_groups_by_page(
    State(state): State<AppState>,
    Query(query): Query<PaginatedQuery<FilterGroupsDto>>,
) -> ServerResult<RestResponseJson<PaginatedData<GroupDto>>> {
    let group_service = GroupService::new(state.db_conn);

    let (groups, total) = group_service.query_groups_by_page(query).await?;
    let records = groups.into_iter().map(GroupDto::from).collect::<Vec<_>>();

    Ok(RestResponse::json(PaginatedData { records, total }))
}

/// Create group
#[utoipa::path(
    operation_id = "createGroup",
    description = "Create group",
    post,
    path = "/createGroup",
    request_body = CreateGroupRequestDto,
    responses(
        (status = OK, description = "ok", body = RestResponseJson<CreateGroupResponseDto>)
    )
)]
pub async fn create_group(
    State(state): State<AppState>,
    Json(params): Json<CreateGroupRequestDto>,
) -> ServerResult<RestResponseJson<CreateGroupResponseDto>> {
    let group_service = GroupService::new(state.db_conn);

    let group_id = group_service
        .create_group(CreateGroupParams {
            name: params.name,
            parent_id: params.parent_id,
            description: params.description,
            ..Default::default()
        })
        .await?;

    Ok(RestResponse::json(CreateGroupResponseDto(group_id)))
}

/// Delete groups
#[utoipa::path(
    operation_id = "deleteGroups",
    description = "Delete groups",
    delete,
    path = "/deleteGroups",
    request_body = DeleteGroupsRequestDto,
    responses(
        (status = OK, description = "ok", body = RestResponseJson<Null>)
    )
)]
pub async fn delete_groups(
    State(state): State<AppState>,
    Json(params): Json<DeleteGroupsRequestDto>,
) -> ServerResult<RestResponseJson<Null>> {
    let group_service = GroupService::new(state.db_conn);
    group_service.delete_groups(params.into()).await?;
    Ok(RestResponse::null())
}

/// Update group
#[utoipa::path(
    operation_id = "updateGroup",
    description = "Update group",
    patch,
    path = "/updateGroup",
    request_body = UpdateGroupRequestDto,
    responses(
        (status = OK, description = "ok", body = RestResponseJson<Null>)
    )
)]
pub async fn update_group(
    State(state): State<AppState>,
    Json(params): Json<UpdateGroupRequestDto>,
) -> ServerResult<RestResponseJson<Null>> {
    let group_service = GroupService::new(state.db_conn);
    group_service.update_group(params.into()).await?;
    Ok(RestResponse::null())
}
