use sea_orm_migration::{prelude::*, schema::*};

use crate::table_manager::TableManager;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, Accounts::Table)
            .create_table(
                Table::create()
                    .col(uuid(Accounts::Id))
                    .col(uuid(Accounts::UserId))
                    .col(string(Accounts::Name))
                    .col(string(Accounts::Kind))
                    .col(integer(Accounts::Balance))
                    .col(string(Accounts::Currency))
                    .col(integer(Accounts::CorrectedBalance))
                    .col(
                        timestamp_with_time_zone(Accounts::CorrectedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, Accounts::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Accounts {
    Table,
    Id,
    UserId,
    Name,
    Kind,
    Balance,
    Currency,
    CorrectedBalance,
    CorrectedAt,
}
