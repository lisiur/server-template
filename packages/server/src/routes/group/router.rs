use app::{
    services::group::{GroupService, create_group::CreateGroupParams},
    utils::query::PaginatedQuery,
};
use axum::{
    Extension, Json, Router,
    extract::Query,
    routing::{delete, get, patch, post},
};
use sea_orm::DatabaseConnection;
use shared::enums::OperationPermission;
use utoipa::OpenApi;

use crate::{
    dto::PaginatedQueryDto,
    extractors::auth_session::AuthSession,
    response::{ApiResponse, Null, PaginatedData, ResponseJson},
    result::ServerResult,
    routes::group::dto::{CreateGroupResponseDto, DeleteGroupsRequestDto, UpdateGroupRequestDto},
};

use super::dto::{CreateGroupRequestDto, FilterGroupsDto, GroupDto};

#[derive(OpenApi)]
#[openapi(paths(create_group, query_groups_by_page, delete_groups, update_group))]
pub(crate) struct ApiDoc;

pub(crate) fn init() -> Router {
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
        (status = OK, description = "ok", body = ResponseJson<PaginatedData<GroupDto>>)
    )
)]
pub async fn query_groups_by_page(
    session: AuthSession,
    Extension(conn): Extension<DatabaseConnection>,
    Query(query): Query<PaginatedQuery<FilterGroupsDto>>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::QueryGroups)?;

    let group_service = GroupService::new(conn);

    let (groups, total) = group_service.query_groups_by_page(query).await?;
    let records = groups.into_iter().map(GroupDto::from).collect::<Vec<_>>();

    Ok(ApiResponse::json(PaginatedData { records, total }))
}

/// Create group
#[utoipa::path(
    operation_id = "createGroup",
    description = "Create group",
    post,
    path = "/createGroup",
    request_body = CreateGroupRequestDto,
    responses(
        (status = OK, description = "ok", body = ResponseJson<CreateGroupResponseDto>)
    )
)]
pub async fn create_group(
    session: AuthSession,
    Extension(conn): Extension<DatabaseConnection>,
    Json(params): Json<CreateGroupRequestDto>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::CreateGroup)?;

    let group_service = GroupService::new(conn);

    let group_id = group_service
        .create_group(CreateGroupParams {
            name: params.name,
            parent_id: params.parent_id,
            description: params.description,
            ..Default::default()
        })
        .await?;

    Ok(ApiResponse::json(CreateGroupResponseDto(group_id)))
}

/// Delete groups
#[utoipa::path(
    operation_id = "deleteGroups",
    description = "Delete groups",
    delete,
    path = "/deleteGroups",
    request_body = DeleteGroupsRequestDto,
    responses(
        (status = OK, description = "ok", body = ResponseJson<Null>)
    )
)]
pub async fn delete_groups(
    session: AuthSession,
    Extension(conn): Extension<DatabaseConnection>,
    Json(params): Json<DeleteGroupsRequestDto>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::DeleteGroup)?;

    let group_service = GroupService::new(conn);
    group_service.delete_groups(params.into()).await?;
    Ok(ApiResponse::null())
}

/// Update group
#[utoipa::path(
    operation_id = "updateGroup",
    description = "Update group",
    patch,
    path = "/updateGroup",
    request_body = UpdateGroupRequestDto,
    responses(
        (status = OK, description = "ok", body = ResponseJson<Null>)
    )
)]
pub async fn update_group(
    session: AuthSession,
    Extension(conn): Extension<DatabaseConnection>,
    Json(params): Json<UpdateGroupRequestDto>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::UpdateGroup)?;

    let group_service = GroupService::new(conn);
    group_service.update_group(params.into()).await?;
    Ok(ApiResponse::null())
}
