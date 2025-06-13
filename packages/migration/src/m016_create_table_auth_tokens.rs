use sea_orm_migration::{prelude::*, schema::*};

use crate::table_manager::TableManager;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, AuthTokens::Table)
            .create_table(
                Table::create()
                    .col(uuid(AuthTokens::Id))
                    .col(string(AuthTokens::Kind))
                    .col(uuid(AuthTokens::RefId))
                    .col(string(AuthTokens::Token))
                    .col(string_null(AuthTokens::Ip))
                    .col(string_null(AuthTokens::Agent))
                    .col(string_null(AuthTokens::Platform))
                    .col(string(AuthTokens::Payload))
                    .col(
                        timestamp_with_time_zone_null(AuthTokens::ExpiredAt)
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?
            .create_index(vec![AuthTokens::Kind])
            .await?
            .create_index(vec![AuthTokens::RefId])
            .await?
            .create_index(vec![AuthTokens::Token])
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, AuthTokens::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum AuthTokens {
    Table,
    Id,
    Kind,
    RefId,
    Token,
    Ip,
    Agent,
    Platform,
    Payload,
    ExpiredAt,
}
