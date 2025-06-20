pub use sea_orm_migration::prelude::*;

mod m000_create_table_init;
mod m001_create_table_settings;
mod m002_create_table_departments;
mod m002_create_table_permission_groups;
mod m002_create_table_permissions;
mod m002_create_table_role_groups;
mod m002_create_table_roles;
mod m002_create_table_user_groups;
mod m002_create_table_users;
mod m003_create_table_relation_permission_groups_departments;
mod m003_create_table_relation_permission_groups_roles;
mod m003_create_table_relation_permission_groups_user_groups;
mod m003_create_table_relation_permission_groups_users;
mod m003_create_table_relation_permissions_departments;
mod m003_create_table_relation_permissions_permission_groups;
mod m003_create_table_relation_permissions_roles;
mod m003_create_table_relation_permissions_user_groups;
mod m003_create_table_relation_permissions_users;
mod m003_create_table_relation_role_groups_departments;
mod m003_create_table_relation_role_groups_user_groups;
mod m003_create_table_relation_role_groups_users;
mod m003_create_table_relation_roles_departments;
mod m003_create_table_relation_roles_role_groups;
mod m003_create_table_relation_roles_user_groups;
mod m003_create_table_relation_roles_users;
mod m003_create_table_relation_users_departments;
mod m003_create_table_relation_users_user_groups;
mod m004_create_table_auth_tokens;
mod m099_seeding_data_init;
mod table_manager;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m000_create_table_init::Migration),
            Box::new(m001_create_table_settings::Migration),
            Box::new(m002_create_table_departments::Migration),
            Box::new(m002_create_table_permission_groups::Migration),
            Box::new(m002_create_table_permissions::Migration),
            Box::new(m002_create_table_role_groups::Migration),
            Box::new(m002_create_table_roles::Migration),
            Box::new(m002_create_table_user_groups::Migration),
            Box::new(m002_create_table_users::Migration),
            Box::new(m003_create_table_relation_permissions_departments::Migration),
            Box::new(m003_create_table_relation_permissions_permission_groups::Migration),
            Box::new(m003_create_table_relation_permissions_roles::Migration),
            Box::new(m003_create_table_relation_permissions_user_groups::Migration),
            Box::new(m003_create_table_relation_permissions_users::Migration),
            Box::new(m003_create_table_relation_role_groups_departments::Migration),
            Box::new(m003_create_table_relation_role_groups_user_groups::Migration),
            Box::new(m003_create_table_relation_role_groups_users::Migration),
            Box::new(m003_create_table_relation_roles_departments::Migration),
            Box::new(m003_create_table_relation_permission_groups_roles::Migration),
            Box::new(m003_create_table_relation_roles_role_groups::Migration),
            Box::new(m003_create_table_relation_roles_user_groups::Migration),
            Box::new(m003_create_table_relation_roles_users::Migration),
            Box::new(m003_create_table_relation_users_departments::Migration),
            Box::new(m003_create_table_relation_users_user_groups::Migration),
            Box::new(m003_create_table_relation_permission_groups_departments::Migration),
            Box::new(m003_create_table_relation_permission_groups_user_groups::Migration),
            Box::new(m003_create_table_relation_permission_groups_users::Migration),
            Box::new(m004_create_table_auth_tokens::Migration),
            Box::new(m099_seeding_data_init::Migration),
        ]
    }
}
