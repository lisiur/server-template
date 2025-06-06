use sea_orm_migration::{prelude::*, schema::*};

use crate::table_manager::TableManager;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, Departments::Table)
            .create_table(
                Table::create()
                    .col(uuid(Departments::Id))
                    .col(string(Departments::Name))
                    .col(string_null(Departments::Description))
                    .col(uuid_null(Departments::ParentId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(Departments::ParentId, Departments::Table, Departments::Id)
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, Departments::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Departments {
    Table,
    Id,
    Name,
    Description,
    ParentId,
}
