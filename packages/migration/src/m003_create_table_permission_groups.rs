use sea_orm_migration::{prelude::*, schema::*};

use crate::table_manager::TableManager;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, PermissionGroups::Table)
            .create_table(
                Table::create()
                    .col(uuid(PermissionGroups::Id))
                    .col(string(PermissionGroups::Name))
                    .col(string_null(PermissionGroups::Description))
                    .col(boolean(PermissionGroups::BuiltIn).default(false))
                    .col(uuid_null(PermissionGroups::ParentId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(
                PermissionGroups::ParentId,
                PermissionGroups::Table,
                PermissionGroups::Id,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, PermissionGroups::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum PermissionGroups {
    Table,
    Id,
    Name,
    Description,
    BuiltIn,
    ParentId,
}
