use sea_orm_migration::{prelude::*, schema::*};

use crate::utils::{create_auto_updated_at_trigger, drop_auto_updated_at_trigger};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Setting::Table)
                    .if_not_exists()
                    .col(pk_auto(Setting::Id))
                    .col(string(Setting::Name))
                    .col(string(Setting::Value))
                    .col(string_null(Setting::Description))
                    .col(timestamp(Setting::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(Setting::UpdatedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(&create_auto_updated_at_trigger(&Setting::Table.to_string()))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Setting::Table).to_owned())
            .await?;

        manager
            .get_connection()
            .execute_unprepared(&drop_auto_updated_at_trigger(&Setting::Table.to_string()))
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Setting {
    Table,
    Id,
    Name,
    Value,
    Description,
    CreatedAt,
    UpdatedAt,
}
