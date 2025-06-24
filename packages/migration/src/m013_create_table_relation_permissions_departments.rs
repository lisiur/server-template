use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m002_create_table_departments::Departments, m004_create_table_permissions::Permissions,
    table_manager::TableManager,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationPermissionsDepartments::Table)
            .primary_key(vec![
                RelationPermissionsDepartments::PermissionId,
                RelationPermissionsDepartments::DepartmentId,
            ])
            .create_table(
                Table::create()
                    .col(uuid(RelationPermissionsDepartments::PermissionId))
                    .col(uuid(RelationPermissionsDepartments::DepartmentId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(
                RelationPermissionsDepartments::PermissionId,
                Permissions::Table,
                Permissions::Id,
            )
            .await?
            .create_foreign_key(
                RelationPermissionsDepartments::DepartmentId,
                Departments::Table,
                Departments::Id,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationPermissionsDepartments::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum RelationPermissionsDepartments {
    Table,
    PermissionId,
    DepartmentId,
}
