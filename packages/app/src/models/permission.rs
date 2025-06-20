use chrono::{DateTime, Utc};
use entity::permissions;
use uuid::Uuid;

pub struct Permission {
    pub id: Uuid,
    pub code: String,
    pub kind: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<permissions::Model> for Permission {
    fn from(model: permissions::Model) -> Self {
        Self {
            id: model.id,
            code: model.code,
            kind: model.kind,
            description: model.description,
            created_at: model.created_at.into(),
            updated_at: model.updated_at.into(),
        }
    }
}
