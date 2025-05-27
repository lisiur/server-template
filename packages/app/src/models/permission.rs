use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Permission {
    pub id: Uuid,
    pub code: String,
    pub kind: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}