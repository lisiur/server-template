use chrono::{DateTime, Utc};
use entity::account_books;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBook {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub currency: String,
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<account_books::Model> for AccountBook {
    fn from(value: account_books::Model) -> Self {
        Self {
            id: value.id,
            owner_id: value.owner_id,
            name: value.name,
            currency: value.currency,
            is_deleted: value.is_deleted,
            created_at: value.created_at.into(),
            updated_at: value.updated_at.into(),
        }
    }
}
