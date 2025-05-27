use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m003_create_table_roles::Roles, m005_create_table_groups::Groups, table_manager::TableManager,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationGroupsRoles::Table)
            .primary_key(vec![
                RelationGroupsRoles::RoleId,
                RelationGroupsRoles::GroupId,
            ])
            .create_table(
                Table::create()
                    .col(uuid(RelationGroupsRoles::RoleId))
                    .col(uuid(RelationGroupsRoles::GroupId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(RelationGroupsRoles::RoleId, Roles::Table, Roles::Id)
            .await?
            .create_foreign_key(RelationGroupsRoles::GroupId, Groups::Table, Groups::Id)
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationGroupsRoles::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum RelationGroupsRoles {
    Table,
    RoleId,
    GroupId,
}
