use sea_orm_migration::{prelude::*, schema::*};

use crate::table_manager::TableManager;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, Applications::Table)
            .create_table(
                Table::create()
                    .col(uuid(Applications::Id))
                    .col(uuid(Applications::AppId))
                    .col(string(Applications::AppSecret))
                    .col(string(Applications::Name))
                    .col(uuid(Applications::Description))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, Applications::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Applications {
    Table,
    Id,
    AppId,
    AppSecret,
    Name,
    Description,
}
