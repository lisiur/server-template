use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m003_create_table_permission_groups::PermissionGroups, m008_create_table_users::Users,
    table_manager::TableManager,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationPermissionGroupsUsers::Table)
            .primary_key(vec![
                RelationPermissionGroupsUsers::PermissionGroupId,
                RelationPermissionGroupsUsers::UserId,
            ])
            .create_table(
                Table::create()
                    .col(uuid(RelationPermissionGroupsUsers::PermissionGroupId))
                    .col(uuid(RelationPermissionGroupsUsers::UserId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(
                RelationPermissionGroupsUsers::PermissionGroupId,
                PermissionGroups::Table,
                PermissionGroups::Id,
            )
            .await?
            .create_foreign_key(
                RelationPermissionGroupsUsers::UserId,
                Users::Table,
                Users::Id,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationPermissionGroupsUsers::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum RelationPermissionGroupsUsers {
    Table,
    PermissionGroupId,
    UserId,
}
