use sea_orm_migration::{prelude::*, schema::*};

use crate::table_manager::TableManager;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RoleGroups::Table)
            .create_table(
                Table::create()
                    .col(uuid(RoleGroups::Id))
                    .col(string(RoleGroups::Name))
                    .col(string_null(RoleGroups::Description))
                    .col(boolean(RoleGroups::BuiltIn).default(false))
                    .col(uuid_null(RoleGroups::ParentId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(RoleGroups::ParentId, RoleGroups::Table, RoleGroups::Id)
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RoleGroups::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum RoleGroups {
    Table,
    Id,
    Name,
    Description,
    BuiltIn,
    ParentId,
}
