use sea_orm_migration::{prelude::*, schema::*};

use crate::table_manager::TableManager;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, Users::Table)
            .create_table(
                Table::create()
                    .col(pk_uuid(Users::Id))
                    .col(string(Users::Account))
                    .col(string_null(Users::Nickname))
                    .col(string_null(Users::RealName))
                    .col(string_null(Users::Phone))
                    .col(string_null(Users::Email))
                    .col(boolean(Users::EmailVerified).default(false))
                    .col(string_null(Users::AvatarUrl))
                    .col(string(Users::Gender))
                    .col(date_null(Users::Birthday))
                    .col(string_null(Users::Bio))
                    .col(string_null(Users::PasswordDigest))
                    .col(string_null(Users::LastLogin))
                    .col(tiny_integer(Users::FailedLoginAttempts).default(0))
                    .to_owned(),
            )
            .await?
            .create_index(vec![Users::Gender])
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, Users::Table)
            .drop_index(vec![Users::Gender])
            .await?
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Users {
    Table,
    Id,
    Account,
    Nickname,
    RealName,
    Phone,
    Email,
    EmailVerified,
    AvatarUrl,
    Gender,
    Birthday,
    Bio,
    PasswordDigest,
    LastLogin,
    FailedLoginAttempts,
}
