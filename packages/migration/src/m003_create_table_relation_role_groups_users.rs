use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m002_create_table_role_groups::RoleGroups, m002_create_table_users::Users,
    table_manager::TableManager,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationRoleGroupsUsers::Table)
            .primary_key(vec![
                RelationRoleGroupsUsers::RoleGroupId,
                RelationRoleGroupsUsers::UserId,
            ])
            .create_table(
                Table::create()
                    .col(uuid(RelationRoleGroupsUsers::RoleGroupId))
                    .col(uuid(RelationRoleGroupsUsers::UserId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(
                RelationRoleGroupsUsers::RoleGroupId,
                RoleGroups::Table,
                RoleGroups::Id,
            )
            .await?
            .create_foreign_key(RelationRoleGroupsUsers::UserId, Users::Table, Users::Id)
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationRoleGroupsUsers::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum RelationRoleGroupsUsers {
    Table,
    RoleGroupId,
    UserId,
}
