use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m002_create_table_permissions::Permissions, m012_create_table_departments::Departments,
    table_manager::TableManager,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationDepartmentsPermissions::Table)
            .primary_key(vec![
                RelationDepartmentsPermissions::DepartmentId,
                RelationDepartmentsPermissions::PermissionId,
            ])
            .create_table(
                Table::create()
                    .col(uuid(RelationDepartmentsPermissions::DepartmentId))
                    .col(uuid(RelationDepartmentsPermissions::PermissionId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(
                RelationDepartmentsPermissions::DepartmentId,
                Departments::Table,
                Departments::Id,
            )
            .await?
            .create_foreign_key(
                RelationDepartmentsPermissions::PermissionId,
                Permissions::Table,
                Permissions::Id,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationDepartmentsPermissions::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum RelationDepartmentsPermissions {
    Table,
    DepartmentId,
    PermissionId,
}
