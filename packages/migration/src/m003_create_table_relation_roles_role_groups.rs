use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m002_create_table_role_groups::RoleGroups, m002_create_table_roles::Roles,
    table_manager::TableManager,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationRolesRoleGroups::Table)
            .primary_key(vec![
                RelationRolesRoleGroups::RoleId,
                RelationRolesRoleGroups::RoleGroupId,
            ])
            .create_table(
                Table::create()
                    .col(uuid(RelationRolesRoleGroups::RoleId))
                    .col(uuid(RelationRolesRoleGroups::RoleGroupId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(RelationRolesRoleGroups::RoleId, Roles::Table, Roles::Id)
            .await?
            .create_foreign_key(
                RelationRolesRoleGroups::RoleGroupId,
                RoleGroups::Table,
                RoleGroups::Id,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationRolesRoleGroups::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum RelationRolesRoleGroups {
    Table,
    RoleId,
    RoleGroupId,
}
