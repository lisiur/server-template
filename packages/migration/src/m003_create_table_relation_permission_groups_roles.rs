use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m002_create_table_permission_groups::PermissionGroups, m002_create_table_roles::Roles,
    table_manager::TableManager,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationPermissionGroupsRoles::Table)
            .primary_key(vec![
                RelationPermissionGroupsRoles::PermissionGroupId,
                RelationPermissionGroupsRoles::RoleId,
            ])
            .create_table(
                Table::create()
                    .col(uuid(RelationPermissionGroupsRoles::PermissionGroupId))
                    .col(uuid(RelationPermissionGroupsRoles::RoleId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(
                RelationPermissionGroupsRoles::PermissionGroupId,
                PermissionGroups::Table,
                PermissionGroups::Id,
            )
            .await?
            .create_foreign_key(
                RelationPermissionGroupsRoles::RoleId,
                Roles::Table,
                Roles::Id,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationPermissionGroupsRoles::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum RelationPermissionGroupsRoles {
    Table,
    PermissionGroupId,
    RoleId,
}
