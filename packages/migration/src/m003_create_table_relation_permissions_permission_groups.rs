use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m002_create_table_permission_groups::PermissionGroups,
    m002_create_table_permissions::Permissions, table_manager::TableManager,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationPermissionsPermissionGroups::Table)
            .primary_key(vec![
                RelationPermissionsPermissionGroups::PermissionId,
                RelationPermissionsPermissionGroups::PermissionGroupId,
            ])
            .create_table(
                Table::create()
                    .col(uuid(RelationPermissionsPermissionGroups::PermissionId))
                    .col(uuid(RelationPermissionsPermissionGroups::PermissionGroupId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(
                RelationPermissionsPermissionGroups::PermissionId,
                Permissions::Table,
                Permissions::Id,
            )
            .await?
            .create_foreign_key(
                RelationPermissionsPermissionGroups::PermissionGroupId,
                PermissionGroups::Table,
                PermissionGroups::Id,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationPermissionsPermissionGroups::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum RelationPermissionsPermissionGroups {
    Table,
    PermissionId,
    PermissionGroupId,
}
