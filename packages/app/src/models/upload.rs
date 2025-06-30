use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use entity::uploads;
use serde::{Deserialize, Serialize};
use shared::{
    enums::UploadStatus,
    utils::{decode_base64, encode_base64},
};
use uuid::Uuid;

use crate::result::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Upload {
    pub id: Uuid,
    pub hash: String,
    pub name: String,
    pub extension: String,
    pub size: i64,
    pub chunk_size: i32,
    pub status: UploadStatus,
    pub merged_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<uploads::Model> for Upload {
    fn from(value: uploads::Model) -> Self {
        Self {
            id: value.id,
            hash: value.hash,
            name: value.name,
            extension: value.extension,
            size: value.size,
            chunk_size: value.chunk_size,
            status: value.status.as_str().try_into().unwrap(),
            merged_at: value.merged_at.map(Into::into),
            created_at: value.created_at.into(),
            updated_at: value.updated_at.into(),
        }
    }
}

impl Upload {
    pub fn mime_type(&self) -> String {
        mime_guess::from_path(self.name.clone())
            .first_or_octet_stream()
            .to_string()
    }

    pub fn chunk_count(&self) -> i64 {
        let size = self.size;
        let chunk_size = self.chunk_size as i64;
        size / chunk_size + if size % chunk_size > 0 { 1 } else { 0 }
    }

    pub fn chunk_path(&self, store_path: &Path, chunk_index: i32) -> PathBuf {
        let parent_dir = self.parent_dir(store_path);
        let chunk_counts = self.chunk_count();
        let width = chunk_counts.to_string().len();
        let chunk_name = format!("{:0width$}.chunk", chunk_index, width = width);
        parent_dir.join(chunk_name)
    }

    pub fn encode_name(name: &str) -> String {
        encode_base64(name.as_bytes())
    }

    pub fn decode_name(name: &str) -> AppResult<String> {
        let bytes = decode_base64(name)?;
        Ok(String::from_utf8_lossy(&bytes).to_string())
    }

    pub fn parent_dir(&self, store_path: &Path) -> PathBuf {
        store_path.join(self.id.to_string())
    }
    pub fn file_path(&self, store_path: &Path) -> PathBuf {
        let file_name = Self::encode_name(&self.name);
        let mut path = self.parent_dir(store_path).join(file_name);
        path.set_extension(&self.extension);
        path
    }
}
