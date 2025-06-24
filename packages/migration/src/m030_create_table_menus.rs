use sea_orm_migration::{prelude::*, schema::*};

use crate::{m029_create_table_applications::Applications, table_manager::TableManager};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, Menus::Table)
            .create_table(
                Table::create()
                    .col(uuid(Menus::Id))
                    .col(uuid_null(Menus::ParentId))
                    .col(string(Menus::Name))
                    .col(string(Menus::Code))
                    .col(string(Menus::Description))
                    .col(uuid(Menus::ApplicationId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(Menus::ParentId, Menus::Table, Menus::Id)
            .await?
            .create_foreign_key(Menus::ApplicationId, Applications::Table, Applications::Id)
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, Menus::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Menus {
    Table,
    Id,
    ParentId,
    Name,
    Code,
    Description,
    ApplicationId,
}
