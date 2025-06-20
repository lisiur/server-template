use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m002_create_table_permissions::Permissions, m002_create_table_roles::Roles,
    table_manager::TableManager,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationPermissionsRoles::Table)
            .primary_key(vec![
                RelationPermissionsRoles::PermissionId,
                RelationPermissionsRoles::RoleId,
            ])
            .create_table(
                Table::create()
                    .col(uuid(RelationPermissionsRoles::PermissionId))
                    .col(uuid(RelationPermissionsRoles::RoleId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(
                RelationPermissionsRoles::PermissionId,
                Permissions::Table,
                Permissions::Id,
            )
            .await?
            .create_foreign_key(RelationPermissionsRoles::RoleId, Roles::Table, Roles::Id)
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
    PermissionId,
    RoleId,
}
