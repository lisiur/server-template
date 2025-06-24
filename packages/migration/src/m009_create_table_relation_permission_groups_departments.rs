use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m002_create_table_departments::Departments,
    m003_create_table_permission_groups::PermissionGroups, table_manager::TableManager,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationPermissionGroupsDepartments::Table)
            .primary_key(vec![
                RelationPermissionGroupsDepartments::PermissionGroupId,
                RelationPermissionGroupsDepartments::DepartmentId,
            ])
            .create_table(
                Table::create()
                    .col(uuid(RelationPermissionGroupsDepartments::PermissionGroupId))
                    .col(uuid(RelationPermissionGroupsDepartments::DepartmentId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(
                RelationPermissionGroupsDepartments::PermissionGroupId,
                PermissionGroups::Table,
                PermissionGroups::Id,
            )
            .await?
            .create_foreign_key(
                RelationPermissionGroupsDepartments::DepartmentId,
                Departments::Table,
                Departments::Id,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationPermissionGroupsDepartments::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum RelationPermissionGroupsDepartments {
    Table,
    PermissionGroupId,
    DepartmentId,
}
