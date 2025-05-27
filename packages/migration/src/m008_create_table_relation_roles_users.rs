use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m003_create_table_roles::Roles, m004_create_table_users::Users, table_manager::TableManager
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationRolesUsers::Table)
            .create_table(
                Table::create()
                    .col(pk_uuid(RelationRolesUsers::Id))
                    .col(uuid(RelationRolesUsers::UserId))
                    .col(uuid(RelationRolesUsers::RoleId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(RelationRolesUsers::UserId, Users::Table, Users::Id)
            .await?
            .create_foreign_key(RelationRolesUsers::RoleId, Roles::Table, Roles::Id)
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
    Id,
    UserId,
    RoleId,
}
