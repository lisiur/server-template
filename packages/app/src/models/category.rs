use chrono::{DateTime, Utc};
use entity::categories;
use serde::{Deserialize, Serialize};
use shared::enums::CategoryType;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: Uuid,
    pub account_book_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub category_type: CategoryType,
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<categories::Model> for Category {
    fn from(value: categories::Model) -> Self {
        Self {
            id: value.id,
            account_book_id: value.account_book_id,
            parent_id: value.parent_id,
            name: value.name,
            category_type: value.category_type.as_str().try_into().unwrap(),
            is_deleted: value.is_deleted,
            created_at: value.created_at.into(),
            updated_at: value.updated_at.into(),
        }
    }
}
