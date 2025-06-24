use sea_orm_migration::{prelude::*, schema::*};

use crate::{m100_create_table_account_books::AccountBooks, table_manager::TableManager};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, Categories::Table)
            .create_table(
                Table::create()
                    .col(uuid(Categories::Id))
                    .col(uuid(Categories::AccountBookId))
                    .col(uuid_null(Categories::ParentId))
                    .col(string(Categories::Name))
                    .col(string(Categories::CategoryType))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(
                Categories::AccountBookId,
                AccountBooks::Table,
                AccountBooks::Id,
            )
            .await?
            .create_foreign_key(Categories::ParentId, Categories::Table, Categories::Id)
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, Categories::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Categories {
    Table,
    Id,
    AccountBookId,
    ParentId,
    Name,
    CategoryType,
}
