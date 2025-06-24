use sea_orm_migration::{prelude::*, schema::*};

use crate::table_manager::TableManager;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, Collaborations::Table)
            .create_table(
                Table::create()
                    .col(uuid(Collaborations::Id))
                    .col(uuid(Collaborations::AccountBookId))
                    .col(uuid(Collaborations::UserId))
                    .col(uuid(Collaborations::RoleId))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, Collaborations::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Collaborations {
    Table,
    Id,
    AccountBookId,
    UserId,
    RoleId,
}
