use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m002_create_table_departments::Departments, m008_create_table_users::Users,
    table_manager::TableManager,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationUsersDepartments::Table)
            .primary_key(vec![
                RelationUsersDepartments::UserId,
                RelationUsersDepartments::DepartmentId,
            ])
            .create_table(
                Table::create()
                    .col(uuid(RelationUsersDepartments::UserId))
                    .col(uuid(RelationUsersDepartments::DepartmentId))
                    .to_owned(),
            )
            .await?
            .create_foreign_key(RelationUsersDepartments::UserId, Users::Table, Users::Id)
            .await?
            .create_foreign_key(
                RelationUsersDepartments::DepartmentId,
                Departments::Table,
                Departments::Id,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        TableManager::new(manager, RelationUsersDepartments::Table)
            .drop_table()
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum RelationUsersDepartments {
    Table,
    UserId,
    DepartmentId,
}
