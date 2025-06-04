use app::{
    services::group::{GroupService, create_group::CreateGroupParams},
    utils::query::{FilterAtom, FilterCondition, SelectQuery},
};
use axum::{
    Json, Router,
    extract::{Query, State},
    routing::{delete, get, post},
};
use utoipa::OpenApi;

use crate::{
    dto::{PaginatedQuery, PaginatedQueryDto},
    group::dto::{CreateGroupResponseDto, DeleteGroupsRequestDto},
    rest::{Null, PaginatedData, RestResponse, RestResponseJson},
    result::ServerResult,
    state::AppState,
};

use super::dto::{CreateGroupRequestDto, GroupDto, GroupFilterDto};

#[derive(OpenApi)]
#[openapi(paths(create_group, query_groups_by_page, delete_groups))]
pub(crate) struct ApiDoc;

pub(crate) fn init() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_group))
        .route("/page", get(query_groups_by_page))
        .route("/delete", delete(delete_groups))
}

#[utoipa::path(
    operation_id = "queryGroupsByPage",
    description = "Query groups by page",
    get,
    path = "/page",
    params(PaginatedQueryDto, GroupFilterDto),
    responses(
        (status = OK, description = "ok", body = RestResponseJson<PaginatedData<GroupDto>>)
    )
)]
/// Query groups by page
pub async fn query_groups_by_page(
    State(state): State<AppState>,
    Query(query): Query<PaginatedQuery<GroupFilterDto>>,
) -> ServerResult<RestResponseJson<PaginatedData<GroupDto>>> {
    let group_service = GroupService::new(state.db_conn);

    let mut select_query = SelectQuery::default().with_cursor(query.cursor());
    if let Some(ref name) = query.data.name {
        if !name.is_empty() {
            select_query.add_atom_filter(FilterAtom {
                field: "name".to_string(),
                condition: FilterCondition::Like(format!("%{name}%")),
            });
        }
    }
    let (groups, total) = group_service.query_groups_by_page(select_query).await?;
    let records = groups.into_iter().map(GroupDto::from).collect::<Vec<_>>();

    Ok(RestResponse::json(PaginatedData { records, total }))
}

#[utoipa::path(
    operation_id = "createGroup",
    description = "Create group",
    post,
    path = "/create",
    request_body = CreateGroupRequestDto,
    responses(
        (status = OK, description = "ok", body = RestResponseJson<CreateGroupResponseDto>)
    )
)]
/// Create group
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

#[utoipa::path(
    operation_id = "deleteGroups",
    description = "Delete groups",
    delete,
    path = "/delete",
    request_body = DeleteGroupsRequestDto,
    responses(
        (status = OK, description = "ok", body = RestResponseJson<Null>)
    )
)]
/// Delete groups
pub async fn delete_groups(
    State(state): State<AppState>,
    Json(params): Json<DeleteGroupsRequestDto>,
) -> ServerResult<RestResponseJson<Null>> {
    let group_service = GroupService::new(state.db_conn);
    group_service.delete_groups(params.into()).await?;
    Ok(RestResponse::null())
}
