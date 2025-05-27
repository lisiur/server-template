use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m002_create_table_permissions::Permissions, m005_create_table_groups::Groups,
    table_manager::TableManager,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationGroupsPermissions::Table)
            .primary_key(vec![
                RelationGroupsPermissions::GroupId,
                RelationGroupsPermissions::PermissionId,
            ])
            .create_table(
                Table::create()
                    .col(uuid(RelationGroupsPermissions::GroupId))
                    .col(uuid(RelationGroupsPermissions::PermissionId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(
                RelationGroupsPermissions::GroupId,
                Groups::Table,
                Groups::Id,
            )
            .await?
            .create_foreign_key(
                RelationGroupsPermissions::PermissionId,
                Permissions::Table,
                Permissions::Id,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationGroupsPermissions::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum RelationGroupsPermissions {
    Table,
    GroupId,
    PermissionId,
}
