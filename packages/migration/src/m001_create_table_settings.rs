use sea_orm_migration::{prelude::*, schema::*};

use crate::table_manager::TableManager;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, Settings::Table)
            .create_table(
                Table::create()
                    .col(pk_auto(Settings::Id))
                    .col(string(Settings::Name))
                    .col(string(Settings::Value))
                    .col(string_null(Settings::Description))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, Settings::Table)
            .drop_table()
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Settings {
    Table,
    Id,
    Name,
    Value,
    Description,
}
