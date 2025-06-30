use std::{collections::HashSet, path::Path};

use entity::{upload_chunks, uploads};
use migration::IntoColumnRef;
use sea_orm::{ActiveValue::Set, IntoActiveModel, prelude::*};
use shared::{enums::UploadStatus, utils::hash_file_md5};

use crate::{
    error::AppException,
    models::upload::Upload,
    result::AppResult,
    services::{upload::UploadService, upload_chunk::UploadChunkService},
    utils::query::{Order, QueryCondition, Sort},
};

pub struct CreateUploadParams {
    pub name: String,
    pub hash: String,
    pub size: i64,
    pub chunk_size: i32,
}

pub struct CreateUploadChunkParams {
    pub upload_id: Uuid,
    pub index: i32,
    pub chunk: Vec<u8>,
}

impl UploadService {
    pub async fn create_upload(&self, params: CreateUploadParams) -> AppResult<Upload> {
        let exist = self.query_upload_by_hash(&params.hash).await?;
        if exist.is_some() {
            return Ok(exist.unwrap());
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

    pub async fn query_missing_chunks(&self, upload_id: Uuid) -> AppResult<Vec<i64>> {
        let upload = self.query_upload_by_id(upload_id).await?;
        let Some(upload) = upload else {
            return Err(AppException::UploadNotFound.into());
        };

        let upload_chunk_service = UploadChunkService::new(self.app.clone());
        let chunks = upload_chunk_service
            .crud
            .find_by_condition(upload_chunks::Column::UploadId.eq(upload_id))
            .await?
            .into_iter()
            .map(|x| x.index as i64)
            .collect::<Vec<_>>();
        let chunk_count = upload.chunk_count();

        let exist: HashSet<i64> = chunks.into_iter().collect();
        let missing = (0..chunk_count)
            .filter(|&i| !exist.contains(&i))
            .collect::<Vec<_>>();

        Ok(missing)
    }
    pub async fn create_upload_chunk(&self, params: CreateUploadChunkParams) -> AppResult<()> {
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
        let exist = upload_chunk_service
            .crud
            .find_by_id((params.upload_id, params.index))
            .await?;
        if exist.is_some() {
            return Err(AppException::UploadChunkAlreadyExists.into());
        }
        upload_chunk_service.crud.create(active_model).await?;
        let chunk_path = upload.chunk_path(&self.app.upload_dir, params.index);
        tokio::fs::write(chunk_path, params.chunk).await?;

        Ok(())
    }

    pub async fn merge_upload_chunks(&self, upload_id: Uuid) -> AppResult<Upload> {
        let upload = self.crud.find_by_id(upload_id).await?;
        let Some(upload_model) = upload else {
            return Err(AppException::UploadNotFound.into());
        };

        let upload: Upload = upload_model.clone().into();

        if upload.status == UploadStatus::Merged {
            return Err(AppException::UploadAlreadyMerged.into());
        }

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
        if upload_chunks.len() != upload.chunk_count() as usize {
            return Err(AppException::UploadChunkIncomplete.into());
        }

        let store_path = &self.app.upload_dir;
        let file_path = upload.file_path(&store_path);
        let hash = upload.hash.clone();
        let chunk_paths = upload_chunks
            .iter()
            .map(|chunk| upload.chunk_path(&store_path, chunk.index))
            .collect::<Vec<_>>();

        let mut dest_file = tokio::fs::File::create(&file_path).await?;

        for chunk_path in &chunk_paths {
            let full_path = self.app.upload_dir.as_path().join(chunk_path);
            let mut chunk_file = tokio::fs::File::open(&full_path).await?;
            tokio::io::copy(&mut chunk_file, &mut dest_file).await?;
        }

        let file_hash = hash_file_md5(&file_path).await;

        if file_hash != hash {
            return Err(AppException::FileHashMismatch.into());
        }

        let mut active_model = upload_model.into_active_model();
        active_model.status = Set(UploadStatus::Merged.to_string());
        let upload = self.crud.update(active_model).await?;

        // delete upload chunks
        upload_chunk_service
            .crud
            .delete_many(upload_chunks::Column::UploadId.eq(upload_id))
            .await?;
        for chunk_path in &chunk_paths {
            if chunk_path.exists() {
                tokio::fs::remove_file(chunk_path).await?;
            }
        }

        Ok(upload.into())
    }
}
