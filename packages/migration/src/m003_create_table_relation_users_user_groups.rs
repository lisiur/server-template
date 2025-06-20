use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m002_create_table_user_groups::UserGroups, m002_create_table_users::Users,
    table_manager::TableManager,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationUsersUserGroups::Table)
            .primary_key(vec![
                RelationUsersUserGroups::UserId,
                RelationUsersUserGroups::UserGroupId,
            ])
            .create_table(
                Table::create()
                    .col(uuid(RelationUsersUserGroups::UserId))
                    .col(uuid(RelationUsersUserGroups::UserGroupId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(RelationUsersUserGroups::UserId, Users::Table, Users::Id)
            .await?
            .create_foreign_key(
                RelationUsersUserGroups::UserGroupId,
                UserGroups::Table,
                UserGroups::Id,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationUsersUserGroups::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum RelationUsersUserGroups {
    Table,
    UserId,
    UserGroupId,
}
