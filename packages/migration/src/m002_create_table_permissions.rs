use sea_orm_migration::{prelude::*, schema::*};

use crate::table_manager::TableManager;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, Permissions::Table)
            .create_table(
                Table::create()
                    .col(uuid(Permissions::Id))
                    .col(string(Permissions::Kind))
                    .col(string(Permissions::Code).unique_key())
                    .col(string_null(Permissions::Description))
                    .to_owned(),
            )
            .await?
            .create_index(vec![Permissions::Kind])
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, Permissions::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Permissions {
    Table,
    Id,
    Kind,
    Code,
    Description,
}
