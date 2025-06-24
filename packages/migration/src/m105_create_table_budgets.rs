use sea_orm_migration::{prelude::*, schema::*};

use crate::table_manager::TableManager;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, Budgets::Table)
            .create_table(
                Table::create()
                    .col(uuid(Budgets::Id))
                    .col(uuid(Budgets::AccountBookId))
                    .col(uuid(Budgets::CategoryId))
                    .col(integer(Budgets::Amount))
                    .col(string(Budgets::CycleType))
                    .col(timestamp_with_time_zone_null(Budgets::StartDate))
                    .col(timestamp_with_time_zone_null(Budgets::EndDate))
                    .col(integer(Budgets::CurrentUsed))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, Budgets::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Budgets {
    Table,
    Id,
    AccountBookId,
    CategoryId,
    Amount,
    CycleType,
    StartDate,
    EndDate,
    CurrentUsed,
}
