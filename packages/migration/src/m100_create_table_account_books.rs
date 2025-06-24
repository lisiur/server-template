use sea_orm_migration::{prelude::*, schema::*};

use crate::{m008_create_table_users::Users, table_manager::TableManager};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, AccountBooks::Table)
            .create_table(
                Table::create()
                    .col(uuid(AccountBooks::Id))
                    .col(uuid(AccountBooks::OwnerId))
                    .col(uuid(AccountBooks::Name))
                    .col(uuid(AccountBooks::Currency))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(AccountBooks::OwnerId, Users::Table, Users::Id)
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, AccountBooks::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum AccountBooks {
    Table,
    Id,
    OwnerId,
    Name,
    Currency,
}
