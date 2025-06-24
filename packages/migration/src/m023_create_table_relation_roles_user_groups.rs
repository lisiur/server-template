use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m006_create_table_roles::Roles, m007_create_table_user_groups::UserGroups,
    table_manager::TableManager,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationRolesUserGroups::Table)
            .primary_key(vec![
                RelationRolesUserGroups::RoleId,
                RelationRolesUserGroups::UserGroupId,
            ])
            .create_table(
                Table::create()
                    .col(uuid(RelationRolesUserGroups::RoleId))
                    .col(uuid(RelationRolesUserGroups::UserGroupId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(RelationRolesUserGroups::RoleId, Roles::Table, Roles::Id)
            .await?
            .create_foreign_key(
                RelationRolesUserGroups::UserGroupId,
                UserGroups::Table,
                UserGroups::Id,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationRolesUserGroups::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum RelationRolesUserGroups {
    Table,
    RoleId,
    UserGroupId,
}
