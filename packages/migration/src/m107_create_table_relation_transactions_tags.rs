use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m104_create_table_tags::Tags, m106_create_table_transactions::Transactions,
    table_manager::TableManager,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationTransactionsTags::Table)
            .primary_key(vec![
                RelationTransactionsTags::TransactionId,
                RelationTransactionsTags::TagId,
            ])
            .create_table(
                Table::create()
                    .col(uuid(RelationTransactionsTags::TransactionId))
                    .col(uuid(RelationTransactionsTags::TagId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(
                RelationTransactionsTags::TransactionId,
                Transactions::Table,
                Transactions::Id,
            )
            .await?
            .create_foreign_key(RelationTransactionsTags::TagId, Tags::Table, Tags::Id)
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationTransactionsTags::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum RelationTransactionsTags {
    Table,
    TransactionId,
    TagId,
}
