use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m003_create_table_permission_groups::PermissionGroups,
    m007_create_table_user_groups::UserGroups, table_manager::TableManager,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationPermissionGroupsUserGroups::Table)
            .primary_key(vec![
                RelationPermissionGroupsUserGroups::PermissionGroupId,
                RelationPermissionGroupsUserGroups::UserGroupId,
            ])
            .create_table(
                Table::create()
                    .col(uuid(RelationPermissionGroupsUserGroups::PermissionGroupId))
                    .col(uuid(RelationPermissionGroupsUserGroups::UserGroupId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(
                RelationPermissionGroupsUserGroups::PermissionGroupId,
                PermissionGroups::Table,
                PermissionGroups::Id,
            )
            .await?
            .create_foreign_key(
                RelationPermissionGroupsUserGroups::UserGroupId,
                UserGroups::Table,
                UserGroups::Id,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationPermissionGroupsUserGroups::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum RelationPermissionGroupsUserGroups {
    Table,
    PermissionGroupId,
    UserGroupId,
}
