use app::{
    models::group::Group,
    services::group::{
        delete_groups::DeleteGroupsParams, query_groups::FilterGroupsParams,
        update_group::UpdateGroupParams,
    },
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
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

impl From<Group> for GroupDto {
    fn from(value: Group) -> Self {
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
#[derive(Debug, Clone, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct FilterGroupsDto {
    pub name: Option<String>,
}

impl From<FilterGroupsDto> for FilterGroupsParams {
    fn from(value: FilterGroupsDto) -> Self {
        FilterGroupsParams { name: value.name }
    }
}

/// Group id list
#[derive(Debug, ToSchema, Deserialize)]
pub struct DeleteGroupsRequestDto(Vec<Uuid>);

impl From<DeleteGroupsRequestDto> for DeleteGroupsParams {
    fn from(value: DeleteGroupsRequestDto) -> Self {
        Self(value.0)
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
