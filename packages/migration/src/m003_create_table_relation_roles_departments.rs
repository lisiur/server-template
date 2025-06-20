use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m002_create_table_departments::Departments, m002_create_table_roles::Roles,
    table_manager::TableManager,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationRolesDepartments::Table)
            .primary_key(vec![
                RelationRolesDepartments::RoleId,
                RelationRolesDepartments::DepartmentId,
            ])
            .create_table(
                Table::create()
                    .col(uuid(RelationRolesDepartments::RoleId))
                    .col(uuid(RelationRolesDepartments::DepartmentId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(RelationRolesDepartments::RoleId, Roles::Table, Roles::Id)
            .await?
            .create_foreign_key(
                RelationRolesDepartments::DepartmentId,
                Departments::Table,
                Departments::Id,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationRolesDepartments::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum RelationRolesDepartments {
    Table,
    RoleId,
    DepartmentId,
}
