use app::{
    models::user_group::UserGroup,
    services::user_group::{
        delete_user_group::DeleteGroupsParams, query_user_group::UserGroupsFilterParams,
        update_user_group::UpdateGroupParams,
    },
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, ToSchema, Deserialize)]
pub struct CreateGroupRequestDto {
    /// group's parent id
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
}

/// Group id
#[derive(Debug, ToSchema, Serialize)]
pub struct CreateGroupResponseDto(pub Uuid);

#[derive(Debug, ToSchema, Serialize)]
pub struct GroupDto {
    pub id: Uuid,
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<UserGroup> for GroupDto {
    fn from(value: UserGroup) -> Self {
        Self {
            id: value.id,
            name: value.name,
            parent_id: value.parent_id,
            description: value.description,
            created_at: value.created_at.into(),
            updated_at: value.updated_at.into(),
        }
    }
}

/// Group filter params
#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct FilterGroupsDto {
    pub name: Option<String>,
}

impl From<FilterGroupsDto> for UserGroupsFilterParams {
    fn from(value: FilterGroupsDto) -> Self {
        UserGroupsFilterParams { name: value.name }
    }
}

/// Group id list
#[derive(Debug, ToSchema, Deserialize)]
pub struct DeleteGroupsRequestDto(Vec<Uuid>);

impl From<DeleteGroupsRequestDto> for DeleteGroupsParams {
    fn from(value: DeleteGroupsRequestDto) -> Self {
        DeleteGroupsParams(value.0)
    }
}

/// Group update params
#[derive(Debug, ToSchema, Deserialize)]
pub struct UpdateGroupRequestDto {
    id: Uuid,
    parent_id: Option<Uuid>,
    name: Option<String>,
    description: Option<String>,
}

impl From<UpdateGroupRequestDto> for UpdateGroupParams {
    fn from(value: UpdateGroupRequestDto) -> Self {
        Self {
            id: value.id,
            parent_id: value.parent_id,
            name: value.name,
            description: value.description,
        }
    }
}
