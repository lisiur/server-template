use app::{
    models::department::Department,
    services::department::{
        create_department::CreateDepartmentParams, delete_departments::DeleteDepartmentsParams,
        query_departments::FilterDepartmentsParams, update_department::UpdateDepartmentParams,
    },
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, ToSchema, Deserialize)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct CreateDepartmentRequestDto {
    /// department's parent id
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
}

impl From<CreateDepartmentRequestDto> for CreateDepartmentParams {
    fn from(dto: CreateDepartmentRequestDto) -> Self {
        CreateDepartmentParams {
            parent_id: dto.parent_id,
            name: dto.name,
            description: dto.description,
        }
    }
}

/// Department id
#[derive(Debug, ToSchema, Serialize)]
pub struct CreateDepartmentResponseDto(pub Uuid);

#[derive(Debug, ToSchema, Serialize)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct DepartmentDto {
    pub id: Uuid,
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Department> for DepartmentDto {
    fn from(value: Department) -> Self {
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

/// Department filter params
#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct FilterDepartmentsDto {
    pub name: Option<String>,
}

impl From<FilterDepartmentsDto> for FilterDepartmentsParams {
    fn from(value: FilterDepartmentsDto) -> Self {
        Self { name: value.name }
    }
}

/// Department id list
#[derive(Debug, ToSchema, Deserialize)]
pub struct DeleteDepartmentsRequestDto(Vec<Uuid>);

impl From<DeleteDepartmentsRequestDto> for DeleteDepartmentsParams {
    fn from(value: DeleteDepartmentsRequestDto) -> Self {
        Self(value.0)
    }
}

/// Department update params
#[derive(Debug, ToSchema, Deserialize)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct UpdateDepartmentRequestDto {
    id: Uuid,
    parent_id: Option<Uuid>,
    name: Option<String>,
    description: Option<String>,
}

impl From<UpdateDepartmentRequestDto> for UpdateDepartmentParams {
    fn from(value: UpdateDepartmentRequestDto) -> Self {
        Self {
            id: value.id,
            parent_id: value.parent_id,
            name: value.name,
            description: value.description,
        }
    }
}
