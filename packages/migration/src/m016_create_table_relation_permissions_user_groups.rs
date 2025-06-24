use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m004_create_table_permissions::Permissions, m007_create_table_user_groups::UserGroups,
    table_manager::TableManager,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationPermissionsUserGroups::Table)
            .primary_key(vec![
                RelationPermissionsUserGroups::PermissionId,
                RelationPermissionsUserGroups::UserGroupId,
            ])
            .create_table(
                Table::create()
                    .col(uuid(RelationPermissionsUserGroups::PermissionId))
                    .col(uuid(RelationPermissionsUserGroups::UserGroupId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(
                RelationPermissionsUserGroups::PermissionId,
                Permissions::Table,
                Permissions::Id,
            )
            .await?
            .create_foreign_key(
                RelationPermissionsUserGroups::UserGroupId,
                UserGroups::Table,
                UserGroups::Id,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationPermissionsUserGroups::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum RelationPermissionsUserGroups {
    Table,
    PermissionId,
    UserGroupId,
}
