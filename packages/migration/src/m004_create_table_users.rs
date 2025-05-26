use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_uuid(User::Id))
                    .col(string(User::Account))
                    .col(string_null(User::Nickname))
                    .col(string_null(User::RealName))
                    .col(string_null(User::Phone))
                    .col(string_null(User::Email))
                    .col(boolean(User::EmailVerified).default(false))
                    .col(string_null(User::AvatarUrl))
                    .col(tiny_integer(User::Gender).default(0))
                    .col(date_null(User::Birthday))
                    .col(string_null(User::Bio))
                    .col(string_null(User::PasswordDigest))
                    .col(string_null(User::LastLogin))
                    .col(tiny_integer(User::FailedLoginAttempts).default(0))
                    .col(timestamp(User::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(User::UpdatedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-user-gender")
                    .table(User::Table)
                    .col(User::Gender)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx-user-gender").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
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
    CreatedAt,
    UpdatedAt,
}
