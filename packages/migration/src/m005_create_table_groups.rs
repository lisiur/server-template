use sea_orm_migration::{prelude::*, schema::*};

use crate::table_manager::TableManager;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, Groups::Table)
            .create_table(
                Table::create()
                    .col(pk_uuid(Groups::Id))
                    .col(string(Groups::Name))
                    .col(string_null(Groups::Description))
                    .col(uuid_null(Groups::ParentId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(Groups::ParentId, Groups::Table, Groups::Id)
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, Groups::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Groups {
    Table,
    Id,
    Name,
    Description,
    ParentId,
}
