use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m003_create_table_roles::Roles, m012_create_table_departments::Departments,
    table_manager::TableManager,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationDepartmentsRoles::Table)
            .primary_key(vec![
                RelationDepartmentsRoles::DepartmentId,
                RelationDepartmentsRoles::RoleId,
            ])
            .create_table(
                Table::create()
                    .col(uuid(RelationDepartmentsRoles::DepartmentId))
                    .col(uuid(RelationDepartmentsRoles::RoleId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(
                RelationDepartmentsRoles::DepartmentId,
                Departments::Table,
                Departments::Id,
            )
            .await?
            .create_foreign_key(RelationDepartmentsRoles::RoleId, Roles::Table, Roles::Id)
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationDepartmentsRoles::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum RelationDepartmentsRoles {
    Table,
    DepartmentId,
    RoleId,
}
