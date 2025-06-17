use app::{
    models::role::Role,
    services::role::{
        create_role::CreateRoleParams, delete_roles::DeleteRolesParams,
        query_roles::FilterRolesParams, update_role::UpdateRoleParams,
    },
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Debug, ToSchema, Serialize)]
pub struct RoleDto {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Role> for RoleDto {
    fn from(value: Role) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            created_at: value.created_at.into(),
            updated_at: value.updated_at.into(),
        }
    }
}

/// Role create params
#[derive(Debug, ToSchema, Deserialize)]
pub struct CreateRoleRequestDto {
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
}

impl From<CreateRoleRequestDto> for CreateRoleParams {
    fn from(dto: CreateRoleRequestDto) -> Self {
        Self {
            name: dto.name,
            description: dto.description,
            parent_id: dto.parent_id,
        }
    }
}

/// Role filter params
#[derive(Debug, Clone, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct RoleFilterDto {
    pub name: Option<String>,
}

impl From<RoleFilterDto> for FilterRolesParams {
    fn from(value: RoleFilterDto) -> Self {
        Self { name: value.name }
    }
}

/// Role id list
#[derive(Debug, ToSchema, Deserialize)]
pub struct DeleteRolesRequestDto(Vec<Uuid>);

impl From<DeleteRolesRequestDto> for DeleteRolesParams {
    fn from(value: DeleteRolesRequestDto) -> Self {
        Self(value.0)
    }
}

/// Role update params
#[derive(Debug, ToSchema, Deserialize)]
pub struct UpdateRoleRequestDto {
    id: Uuid,
    name: Option<String>,
    description: Option<String>,
}

impl From<UpdateRoleRequestDto> for UpdateRoleParams {
    fn from(value: UpdateRoleRequestDto) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
        }
    }
}
