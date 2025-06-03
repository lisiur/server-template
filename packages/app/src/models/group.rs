use chrono::{DateTime, Utc};
use entity::groups;
use uuid::Uuid;

pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<groups::Model> for Group {
    fn from(model: groups::Model) -> Self {
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
