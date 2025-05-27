use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m004_create_table_users::Users, m005_create_table_groups::Groups, table_manager::TableManager,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationGroupsUsers::Table)
            .create_table(
                Table::create()
                    .col(pk_uuid(RelationGroupsUsers::Id))
                    .col(uuid(RelationGroupsUsers::UserId))
                    .col(uuid(RelationGroupsUsers::GroupId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(RelationGroupsUsers::UserId, Users::Table, Users::Id)
            .await?
            .create_foreign_key(RelationGroupsUsers::GroupId, Groups::Table, Groups::Id)
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationGroupsUsers::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum RelationGroupsUsers {
    Table,
    Id,
    UserId,
    GroupId,
}
