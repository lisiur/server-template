use sea_orm_migration::{prelude::*, schema::*};

use crate::table_manager::TableManager;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, Roles::Table)
            .create_table(
                Table::create()
                    .col(uuid(Roles::Id))
                    .col(string(Roles::Name))
                    .col(boolean(Roles::BuiltIn))
                    .col(string_null(Roles::Description))
                    .col(uuid_null(Roles::ParentId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(Roles::ParentId, Roles::Table, Roles::Id)
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, Roles::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Roles {
    Table,
    Id,
    Name,
    BuiltIn,
    Description,
    ParentId,
}
