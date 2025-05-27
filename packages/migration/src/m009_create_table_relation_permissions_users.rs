use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m002_create_table_permissions::Permissions, m004_create_table_users::Users,
    table_manager::TableManager,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationPermissionsUsers::Table)
            .primary_key(vec![
                RelationPermissionsUsers::UserId,
                RelationPermissionsUsers::PermissionId,
            ])
            .create_table(
                Table::create()
                    .col(uuid(RelationPermissionsUsers::UserId))
                    .col(uuid(RelationPermissionsUsers::PermissionId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(RelationPermissionsUsers::UserId, Users::Table, Users::Id)
            .await?
            .create_foreign_key(
                RelationPermissionsUsers::PermissionId,
                Permissions::Table,
                Permissions::Id,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationPermissionsUsers::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum RelationPermissionsUsers {
    Table,
    UserId,
    PermissionId,
}
