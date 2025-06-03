use app::{
    models::permission::Permission, services::permission::create_permission::CreatePermissionParams,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Debug, ToSchema, Deserialize)]
pub struct CreatePermissionDto {
    pub kind: String,
    pub code: String,
    pub description: Option<String>,
}

impl From<CreatePermissionDto> for CreatePermissionParams {
    fn from(dto: CreatePermissionDto) -> Self {
        CreatePermissionParams {
            kind: dto.kind,
            code: dto.code,
            description: dto.description,
        }
    }
}

#[derive(Debug, ToSchema, Serialize)]
pub struct PermissionDto {
    pub id: Uuid,
    pub kind: String,
    pub code: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Permission> for PermissionDto {
    fn from(value: Permission) -> Self {
        Self {
            id: value.id,
            kind: value.kind,
            code: value.code,
            description: value.description,
            created_at: value.created_at.into(),
            updated_at: value.updated_at.into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct PermissionFilterDto {
    pub kind: Option<String>,
}
