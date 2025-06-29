use chrono::{DateTime, Utc};
use entity::upload_chunks;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadChunk {
    pub id: Uuid,
    pub upload_id: Uuid,
    pub index: i32,
    pub size: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<upload_chunks::Model> for UploadChunk {
    fn from(value: upload_chunks::Model) -> Self {
        Self {
            id: value.id,
            upload_id: value.upload_id,
            index: value.index,
            size: value.size,
            created_at: value.created_at.into(),
            updated_at: value.updated_at.into(),
        }
    }
}
