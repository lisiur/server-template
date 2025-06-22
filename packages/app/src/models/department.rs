use chrono::{DateTime, Utc};
use entity::departments;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;


#[derive(ToSchema, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct Department {
    pub id: Uuid,
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<departments::Model> for Department {
    fn from(model: departments::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            parent_id: model.parent_id,
            description: model.description,
            created_at: model.created_at.into(),
            updated_at: model.updated_at.into(),
        }
    }
}
