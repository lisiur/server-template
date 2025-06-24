use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m100_create_table_account_books::AccountBooks, m102_create_table_accounts::Accounts,
    table_manager::TableManager,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationAccountBooksAccounts::Table)
            .primary_key(vec![
                RelationAccountBooksAccounts::AccountBookId,
                RelationAccountBooksAccounts::AccountId,
            ])
            .create_table(
                Table::create()
                    .col(uuid(RelationAccountBooksAccounts::AccountBookId))
                    .col(uuid(RelationAccountBooksAccounts::AccountId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(
                RelationAccountBooksAccounts::AccountBookId,
                AccountBooks::Table,
                AccountBooks::Id,
            )
            .await?
            .create_foreign_key(
                RelationAccountBooksAccounts::AccountId,
                Accounts::Table,
                Accounts::Id,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationAccountBooksAccounts::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum RelationAccountBooksAccounts {
    Table,
    AccountBookId,
    AccountId,
}
