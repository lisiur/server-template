use chrono::{DateTime, Utc};
use entity::permission_groups;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(ToSchema, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct PermissionGroup {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<permission_groups::Model> for PermissionGroup {
    fn from(value: permission_groups::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            parent_id: value.parent_id,
            created_at: value.created_at.into(),
            updated_at: value.updated_at.into(),
        }
    }
}
