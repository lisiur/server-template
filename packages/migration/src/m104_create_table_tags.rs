use sea_orm_migration::{prelude::*, schema::*};

use crate::{m100_create_table_account_books::AccountBooks, table_manager::TableManager};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, Tags::Table)
            .create_table(
                Table::create()
                    .col(uuid(Tags::Id))
                    .col(uuid(Tags::AccountBookId))
                    .col(string(Tags::Name))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(Tags::AccountBookId, AccountBooks::Table, AccountBooks::Id)
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, Tags::Table).drop_table().await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Tags {
    Table,
    Id,
    AccountBookId,
    Name,
}
