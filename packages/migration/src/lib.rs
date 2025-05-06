pub use sea_orm_migration::prelude::*;

mod m00000000_000000_create_table_init;
mod m00000000_000001_create_table_users;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m00000000_000000_create_table_init::Migration),
            Box::new(m00000000_000001_create_table_users::Migration),
        ]
    }
}
