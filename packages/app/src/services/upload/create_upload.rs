use std::path::Path;

use entity::{upload_chunks, uploads};
use migration::IntoColumnRef;
use shared::{enums::UploadStatus, utils::hash_file_md5};

use crate::{
    error::AppException,
    models::{upload::Upload, upload_chunk::UploadChunk},
    result::AppResult,
    services::{upload::UploadService, upload_chunk::UploadChunkService},
    utils::query::{Order, QueryCondition, Sort},
};
use sea_orm::{ActiveValue::Set, prelude::*};

pub struct CreateUploadParams {
    pub name: String,
    pub hash: String,
    pub size: i64,
    pub chunk_size: i32,
}

pub struct CreateUploadChunkParams {
    pub upload_id: Uuid,
    pub chunk: Vec<u8>,
    pub index: i32,
}

impl UploadService {
    pub async fn create_upload(&self, params: CreateUploadParams) -> AppResult<Upload> {
        let exist = self.query_upload_by_hash(&params.hash).await?.is_some();
        if exist {
            return Err(AppException::AlreadyExists.into());
        }

        let id = Uuid::new_v4();

        let extension = Path::new(&params.name)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();

        let active_model = uploads::ActiveModel {
            id: Set(id),
            hash: Set(params.hash),
            name: Set(params.name),
            extension: Set(extension),
            size: Set(params.size),
            chunk_size: Set(params.chunk_size),
            status: Set(UploadStatus::Uploading.to_string()),
            ..Default::default()
        };
        let model = self.crud.create(active_model).await?;

        let upload: Upload = model.into();
        let parent_dir = upload.parent_dir(&self.app.upload_dir);
        std::fs::create_dir_all(&parent_dir)?;

        Ok(upload)
    }

    pub async fn create_upload_chunk(
        &self,
        params: CreateUploadChunkParams,
    ) -> AppResult<UploadChunk> {
        let upload = self.crud.find_by_id(params.upload_id).await?;
        let Some(upload) = upload else {
            return Err(AppException::UploadNotFound.into());
        };
        let upload: Upload = upload.into();

        let upload_chunk_service = UploadChunkService::new(self.app.clone());

        let id = Uuid::new_v4();
        let active_model = upload_chunks::ActiveModel {
            upload_id: Set(params.upload_id),
            id: Set(id),
            index: Set(params.index),
            size: Set(params.chunk.len() as i32),
            ..Default::default()
        };
        let model = upload_chunk_service.crud.create(active_model).await?;
        let chunk_path = upload.chunk_path(&self.app.upload_dir, params.index);
        tokio::fs::write(chunk_path, params.chunk).await?;

        Ok(model.into())
    }

    pub async fn merge_upload_chunks(&self, upload_id: Uuid) -> AppResult<Upload> {
        let upload = self.query_upload_by_id(upload_id).await?;
        let Some(upload) = upload else {
            return Err(AppException::UploadNotFound.into());
        };
        let upload_chunk_service = UploadChunkService::new(self.app.clone());
        let query_condition: QueryCondition = upload_chunks::Column::UploadId.eq(upload_id).into();
        let query_condition = query_condition.with_orders(vec![Sort {
            column_ref: upload_chunks::Column::Index.into_column_ref(),
            order: Order::Asc,
        }]);
        let upload_chunks = upload_chunk_service
            .crud
            .find_by_condition(query_condition)
            .await?;
        if upload_chunks.len() != upload.chunk_counts() as usize {
            return Err(AppException::UploadChunkIncomplete.into());
        }

        let store_path = &self.app.upload_dir;
        let file_path = self
            .app
            .upload_dir
            .to_path_buf()
            .join(upload.file_path(&store_path));
        let hash = upload.hash.clone();
        let chunk_paths = upload_chunks
            .iter()
            .map(|chunk| upload.chunk_path(&store_path, chunk.index))
            .collect::<Vec<_>>();

        let mut dest_file = tokio::fs::File::create(&file_path).await?;

        for chunk_path in chunk_paths {
            let mut chunk_file = tokio::fs::File::open(&chunk_path).await?;
            // 将当前块的内容复制到目标文件
            tokio::io::copy(&mut chunk_file, &mut dest_file).await?;
        }

        let file_hash = hash_file_md5(&file_path).await;

        if file_hash != hash {
            return Err(AppException::FileHashMismatch.into());
        }

        Ok(upload)
    }
}
