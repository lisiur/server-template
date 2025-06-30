use sea_orm_migration::{prelude::*, schema::*};

use crate::{m032_create_table_uploads::Uploads, table_manager::TableManager};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, UploadChunks::Table)
            .primary_key(vec![UploadChunks::UploadId, UploadChunks::Index])
            .create_table(
                Table::create()
                    .col(uuid(UploadChunks::Id))
                    .col(uuid(UploadChunks::UploadId))
                    .col(integer(UploadChunks::Index))
                    .col(integer(UploadChunks::Size))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(UploadChunks::UploadId, Uploads::Table, Uploads::Id)
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, UploadChunks::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum UploadChunks {
    Table,
    Id,
    UploadId,
    Index,
    Size,
}
