use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Permission::Table)
                    .if_not_exists()
                    .col(pk_uuid(Permission::Id))
                    .col(string(Permission::Kind))
                    .col(string(Permission::Code).unique_key())
                    .col(string_null(Permission::Description))
                    .col(timestamp(Permission::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(Permission::UpdatedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Permission::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Permission {
    Table,
    Id,
    Kind,
    Code,
    Description,
    CreatedAt,
    UpdatedAt,
}
