use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m004_create_table_users::Users, m012_create_table_departments::Departments,
    table_manager::TableManager,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationDepartmentsUsers::Table)
            .primary_key(vec![
                RelationDepartmentsUsers::DepartmentId,
                RelationDepartmentsUsers::UserId,
            ])
            .create_table(
                Table::create()
                    .col(uuid(RelationDepartmentsUsers::DepartmentId))
                    .col(uuid(RelationDepartmentsUsers::UserId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(
                RelationDepartmentsUsers::DepartmentId,
                Departments::Table,
                Departments::Id,
            )
            .await?
            .create_foreign_key(RelationDepartmentsUsers::UserId, Users::Table, Users::Id)
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationDepartmentsUsers::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum RelationDepartmentsUsers {
    Table,
    DepartmentId,
    UserId,
}
