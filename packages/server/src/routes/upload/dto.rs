use app::{
    models::upload::Upload,
    services::upload::create_upload::{CreateUploadChunkParams, CreateUploadParams},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared::enums::UploadStatus;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, ToSchema, Serialize)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct UploadDto {
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

impl From<Upload> for UploadDto {
    fn from(value: Upload) -> Self {
        Self {
            id: value.id,
            hash: value.hash,
            name: value.name,
            extension: value.extension,
            size: value.size,
            chunk_size: value.chunk_size,
            status: value.status,
            merged_at: value.merged_at,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, ToSchema, Deserialize)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct CreateUploadDto {
    pub name: String,
    pub hash: String,
    pub size: i64,
    pub chunk_size: i32,
}

impl From<CreateUploadDto> for CreateUploadParams {
    fn from(value: CreateUploadDto) -> Self {
        Self {
            name: value.name,
            hash: value.hash,
            size: value.size,
            chunk_size: value.chunk_size,
        }
    }
}

#[derive(Debug, ToSchema, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadFileChunkDto {
    pub upload_id: Uuid,
    pub chunk: Vec<u8>,
    pub index: i32,
}

impl From<UploadFileChunkDto> for CreateUploadChunkParams {
    fn from(value: UploadFileChunkDto) -> Self {
        Self {
            upload_id: value.upload_id,
            chunk: value.chunk,
            index: value.index,
        }
    }
}

#[derive(Debug, ToSchema, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MergeFileChunksDto {
    pub upload_id: Uuid,
}
