use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m002_create_table_permissions::Permissions, m003_create_table_roles::Roles,
    table_manager::TableManager,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationRolesPermissions::Table)
            .create_table(
                Table::create()
                    .table(RelationRolesPermissions::Table)
                    .if_not_exists()
                    .col(pk_uuid(RelationRolesPermissions::Id))
                    .col(uuid(RelationRolesPermissions::RoleId))
                    .col(uuid(RelationRolesPermissions::PermissionId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(RelationRolesPermissions::RoleId, Roles::Table, Roles::Id)
            .await?
            .create_foreign_key(
                RelationRolesPermissions::PermissionId,
                Permissions::Table,
                Permissions::Id,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationRolesPermissions::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum RelationRolesPermissions {
    Table,
    Id,
    RoleId,
    PermissionId,
}
