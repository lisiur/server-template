use app::services::user_group::{
    UserGroupService, create_user_group::CreateGroupParams, query_user_group::UserGroupsOrderField,
};
use axum::Json;
use shared::enums::OperationPermission;
use utoipa::OpenApi;

use crate::{
    dto::PageableQueryDto,
    extractors::{app_service::AppService, session::Session},
    init_router,
    response::{ApiResponse, Null, PaginatedData, ResponseJson},
    result::ServerResult,
    routes::user_group::dto::{
        CreateGroupResponseDto, DeleteGroupsRequestDto, UpdateGroupRequestDto,
    },
};

use super::dto::{CreateGroupRequestDto, FilterGroupsDto, GroupDto};

#[derive(OpenApi)]
#[openapi(paths(create_user_group, query_groups_by_page, delete_groups, update_group))]
pub(crate) struct ApiDoc;
init_router!(
    create_user_group,
    query_groups_by_page,
    delete_groups,
    update_group
);

/// Query groups by page
#[utoipa::path(
    operation_id = "queryGroupsByPage",
    description = "Query groups by page",
    post,
    path = "/queryGroupsByPage",
    request_body = PageableQueryDto<FilterGroupsDto, UserGroupsOrderField>,
    responses(
        (status = OK, description = "ok", body = ResponseJson<PaginatedData<GroupDto>>)
    )
)]
pub async fn query_groups_by_page(
    session: Session,
    user_group_service: AppService<UserGroupService>,
    Json(params): Json<PageableQueryDto<FilterGroupsDto, UserGroupsOrderField>>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::QueryGroups)?;

    let (groups, total) = user_group_service
        .query_user_groups_by_page(params.into())
        .await?;
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
pub async fn create_user_group(
    session: Session,
    user_group_service: AppService<UserGroupService>,
    Json(params): Json<CreateGroupRequestDto>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::CreateGroup)?;

    let group_id = user_group_service
        .create_user_group(CreateGroupParams {
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
    session: Session,
    user_group_service: AppService<UserGroupService>,
    Json(params): Json<DeleteGroupsRequestDto>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::DeleteGroup)?;

    user_group_service.delete_user_groups(params.into()).await?;
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
    session: Session,
    user_group_service: AppService<UserGroupService>,
    Json(params): Json<UpdateGroupRequestDto>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::UpdateGroup)?;

    user_group_service.update_user_group(params.into()).await?;
    Ok(ApiResponse::null())
}
