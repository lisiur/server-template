use app::{models::group::Group, services::group::create_group::DeleteGroupsParams};
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

#[derive(Debug, ToSchema, Serialize)]
/// Group id
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

#[derive(Debug, Clone, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GroupFilterDto {
    pub name: Option<String>,
}

#[derive(Debug, ToSchema, Deserialize)]
/// Group id list
pub struct DeleteGroupsRequestDto(Vec<Uuid>);

impl From<DeleteGroupsRequestDto> for DeleteGroupsParams {
    fn from(value: DeleteGroupsRequestDto) -> Self {
        Self(value.0)
    }
}
