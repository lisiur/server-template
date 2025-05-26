pub use sea_orm_migration::prelude::*;

mod utils;
mod m000_create_table_init;
mod m001_create_table_setting;
mod m002_create_table_permission;
mod m003_create_table_roles;
mod m004_create_table_users;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m000_create_table_init::Migration),
            Box::new(m001_create_table_setting::Migration),
            Box::new(m002_create_table_permission::Migration),
            Box::new(m003_create_table_roles::Migration),
            Box::new(m004_create_table_users::Migration),
        ]
    }
}
