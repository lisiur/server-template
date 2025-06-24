use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m008_create_table_users::Users, m100_create_table_account_books::AccountBooks,
    m101_create_table_categories::Categories, table_manager::TableManager,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, Transactions::Table)
            .create_table(
                Table::create()
                    .col(uuid(Transactions::Id))
                    .col(uuid(Transactions::AccountBookId))
                    .col(uuid_null(Transactions::CategoryId))
                    .col(string(Transactions::TransferType))
                    .col(uuid_null(Transactions::TransferFrom))
                    .col(uuid_null(Transactions::TransferTo))
                    .col(integer(Transactions::Amount))
                    .col(string(Transactions::Remarks))
                    .col(boolean(Transactions::CountAsIncome))
                    .col(boolean(Transactions::CountAsExpenditure))
                    .col(uuid(Transactions::CreatedBy))
                    .col(uuid(Transactions::UpdatedBy))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(
                Transactions::AccountBookId,
                AccountBooks::Table,
                AccountBooks::Id,
            )
            .await?
            .create_foreign_key(Transactions::CategoryId, Categories::Table, Categories::Id)
            .await?
            .create_foreign_key(
                Transactions::TransferFrom,
                AccountBooks::Table,
                AccountBooks::Id,
            )
            .await?
            .create_foreign_key(
                Transactions::TransferTo,
                AccountBooks::Table,
                AccountBooks::Id,
            )
            .await?
            .create_foreign_key(Transactions::CreatedBy, Users::Table, Users::Id)
            .await?
            .create_foreign_key(Transactions::UpdatedBy, Users::Table, Users::Id)
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, Transactions::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Transactions {
    Table,
    Id,
    AccountBookId,
    CategoryId,
    TransferType,
    TransferFrom,
    TransferTo,
    Amount,
    Remarks,
    CountAsIncome,
    CountAsExpenditure,
    CreatedBy,
    UpdatedBy,
}
