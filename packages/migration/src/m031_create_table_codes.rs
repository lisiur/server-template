use sea_orm_migration::{prelude::*, schema::*};

use crate::table_manager::TableManager;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, Codes::Table)
            .create_table(
                Table::create()
                    .col(uuid(Codes::Id))
                    .col(string(Codes::ParentId))
                    .col(uuid(Codes::Name))
                    .col(uuid(Codes::Code))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, Codes::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Codes {
    Table,
    Id,
    ParentId,
    Name,
    Code,
}
