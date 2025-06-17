use chrono::{DateTime, Utc};
use entity::roles;
use uuid::Uuid;

pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<roles::Model> for Role {
    fn from(value: roles::Model) -> Self {
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
