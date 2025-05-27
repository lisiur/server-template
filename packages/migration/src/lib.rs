pub use sea_orm_migration::prelude::*;

mod table_manager;
mod m000_create_table_init;
mod m001_create_table_settings;
mod m002_create_table_permissions;
mod m003_create_table_roles;
mod m004_create_table_users;
mod m005_create_table_relation_roles_permissions;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m000_create_table_init::Migration),
            Box::new(m001_create_table_settings::Migration),
            Box::new(m002_create_table_permissions::Migration),
            Box::new(m003_create_table_roles::Migration),
            Box::new(m004_create_table_users::Migration),
            Box::new(m005_create_table_relation_roles_permissions::Migration),
        ]
    }
}
