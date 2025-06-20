use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m002_create_table_role_groups::RoleGroups, m002_create_table_user_groups::UserGroups,
    table_manager::TableManager,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationRoleGroupsUserGroups::Table)
            .primary_key(vec![
                RelationRoleGroupsUserGroups::RoleGroupId,
                RelationRoleGroupsUserGroups::UserGroupId,
            ])
            .create_table(
                Table::create()
                    .col(uuid(RelationRoleGroupsUserGroups::RoleGroupId))
                    .col(uuid(RelationRoleGroupsUserGroups::UserGroupId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(
                RelationRoleGroupsUserGroups::RoleGroupId,
                RoleGroups::Table,
                RoleGroups::Id,
            )
            .await?
            .create_foreign_key(
                RelationRoleGroupsUserGroups::UserGroupId,
                UserGroups::Table,
                UserGroups::Id,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationRoleGroupsUserGroups::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum RelationRoleGroupsUserGroups {
    Table,
    RoleGroupId,
    UserGroupId,
}
