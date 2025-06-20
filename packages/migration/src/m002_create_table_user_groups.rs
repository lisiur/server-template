use sea_orm_migration::{prelude::*, schema::*};

use crate::table_manager::TableManager;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, UserGroups::Table)
            .create_table(
                Table::create()
                    .col(uuid(UserGroups::Id))
                    .col(string(UserGroups::Name))
                    .col(string_null(UserGroups::Description))
                    .col(uuid_null(UserGroups::ParentId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(UserGroups::ParentId, UserGroups::Table, UserGroups::Id)
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, UserGroups::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum UserGroups {
    Table,
    Id,
    Name,
    Description,
    ParentId,
}
