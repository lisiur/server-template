use sea_orm_migration::{prelude::*, schema::*};

use crate::table_manager::TableManager;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, Uploads::Table)
            .create_table(
                Table::create()
                    .col(uuid(Uploads::Id))
                    .col(string(Uploads::Hash))
                    .col(string(Uploads::Name))
                    .col(string(Uploads::Extension))
                    .col(big_integer(Uploads::Size))
                    .col(integer(Uploads::ChunkSize))
                    .col(string(Uploads::Status))
                    .col(timestamp_with_time_zone_null(Uploads::MergedAt))
                    .to_owned(),
            )
            .await?
            .create_index(vec![Uploads::Hash])
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, Uploads::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Uploads {
    Table,
    Id,
    Hash,
    Name,
    Extension,
    Size,
    ChunkSize,
    Status,
    MergedAt,
}
