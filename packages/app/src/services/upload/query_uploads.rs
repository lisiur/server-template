use entity::uploads;

use crate::{models::upload::Upload, result::AppResult, services::upload::UploadService};
use sea_orm::prelude::*;

impl UploadService {
    pub async fn query_upload_by_id(&self, id: Uuid) -> AppResult<Option<Upload>> {
        let model = self.crud.find_by_id(id).await?;

        Ok(model.map(Into::into))
    }

    pub async fn query_upload_by_hash(&self, hash: &str) -> AppResult<Option<Upload>> {
        let model = self
            .crud
            .find_one_by_condition(uploads::Column::Hash.eq(hash))
            .await?;

        Ok(model.map(Into::into))
    }
}
