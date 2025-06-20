use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m002_create_table_departments::Departments, m002_create_table_role_groups::RoleGroups,
    table_manager::TableManager,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationRoleGroupsDepartments::Table)
            .primary_key(vec![
                RelationRoleGroupsDepartments::RoleGroupId,
                RelationRoleGroupsDepartments::DepartmentId,
            ])
            .create_table(
                Table::create()
                    .col(uuid(RelationRoleGroupsDepartments::RoleGroupId))
                    .col(uuid(RelationRoleGroupsDepartments::DepartmentId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(
                RelationRoleGroupsDepartments::RoleGroupId,
                RoleGroups::Table,
                RoleGroups::Id,
            )
            .await?
            .create_foreign_key(
                RelationRoleGroupsDepartments::DepartmentId,
                Departments::Table,
                Departments::Id,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationRoleGroupsDepartments::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum RelationRoleGroupsDepartments {
    Table,
    RoleGroupId,
    DepartmentId,
}
