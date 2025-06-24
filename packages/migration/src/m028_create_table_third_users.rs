use sea_orm_migration::{prelude::*, schema::*};

use crate::table_manager::TableManager;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, ThirdUsers::Table)
            .primary_key(vec![ThirdUsers::UserId, ThirdUsers::Platform])
            .create_table(
                Table::create()
                    .col(uuid(ThirdUsers::UserId))
                    .col(string(ThirdUsers::Platform))
                    .col(string(ThirdUsers::OpenId))
                    .col(string_null(ThirdUsers::UnionId))
                    .col(string_null(ThirdUsers::AccessToken))
                    .col(string_null(ThirdUsers::Payload))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, ThirdUsers::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum ThirdUsers {
    Table,
    UserId,
    Platform,
    OpenId,
    UnionId,
    AccessToken,
    Payload,
}
