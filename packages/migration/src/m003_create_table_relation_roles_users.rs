use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m002_create_table_roles::Roles, m002_create_table_users::Users, table_manager::TableManager,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationRolesUsers::Table)
            .primary_key(vec![RelationRolesUsers::RoleId, RelationRolesUsers::UserId])
            .create_table(
                Table::create()
                    .col(uuid(RelationRolesUsers::RoleId))
                    .col(uuid(RelationRolesUsers::UserId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(RelationRolesUsers::RoleId, Roles::Table, Roles::Id)
            .await?
            .create_foreign_key(RelationRolesUsers::UserId, Users::Table, Users::Id)
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationRolesUsers::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum RelationRolesUsers {
    Table,
    RoleId,
    UserId,
}
