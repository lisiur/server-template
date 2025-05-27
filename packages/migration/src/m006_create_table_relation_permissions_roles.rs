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
        TableManager::new(manager, RelationPermissionsRoles::Table)
            .create_table(
                Table::create()
                    .col(pk_uuid(RelationPermissionsRoles::Id))
                    .col(uuid(RelationPermissionsRoles::RoleId))
                    .col(uuid(RelationPermissionsRoles::PermissionId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(RelationPermissionsRoles::RoleId, Roles::Table, Roles::Id)
            .await?
            .create_foreign_key(
                RelationPermissionsRoles::PermissionId,
                Permissions::Table,
                Permissions::Id,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationPermissionsRoles::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum RelationPermissionsRoles {
    Table,
    Id,
    RoleId,
    PermissionId,
}
