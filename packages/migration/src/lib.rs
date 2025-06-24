pub use sea_orm_migration::prelude::*;

mod m000_create_table_init;
mod m001_create_table_settings;
mod m002_create_table_departments;
mod m003_create_table_permission_groups;
mod m004_create_table_permissions;
mod m005_create_table_role_groups;
mod m006_create_table_roles;
mod m007_create_table_user_groups;
mod m008_create_table_users;
mod m009_create_table_relation_permission_groups_departments;
mod m010_create_table_relation_permission_groups_roles;
mod m011_create_table_relation_permission_groups_user_groups;
mod m012_create_table_relation_permission_groups_users;
mod m013_create_table_relation_permissions_departments;
mod m014_create_table_relation_permissions_permission_groups;
mod m015_create_table_relation_permissions_roles;
mod m016_create_table_relation_permissions_user_groups;
mod m017_create_table_relation_permissions_users;
mod m018_create_table_relation_role_groups_departments;
mod m019_create_table_relation_role_groups_user_groups;
mod m020_create_table_relation_role_groups_users;
mod m021_create_table_relation_roles_departments;
mod m022_create_table_relation_roles_role_groups;
mod m023_create_table_relation_roles_user_groups;
mod m024_create_table_relation_roles_users;
mod m025_create_table_relation_users_departments;
mod m026_create_table_relation_users_user_groups;
mod m027_create_table_auth_tokens;
mod m028_create_table_third_users;
mod m029_create_table_applications;
mod m030_create_table_menus;
mod m031_create_table_codes;
mod m099_seeding_data_init;
mod m100_create_table_account_books;
mod m101_create_table_categories;
mod m102_create_table_accounts;
mod m103_create_table_relation_account_books_accounts;
mod m104_create_table_tags;
mod m105_create_table_budgets;
mod m106_create_table_transactions;
mod m107_create_table_relation_transactions_tags;
mod m108_create_table_collaborations;
mod table_manager;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m000_create_table_init::Migration),
            Box::new(m001_create_table_settings::Migration),
            Box::new(m002_create_table_departments::Migration),
            Box::new(m003_create_table_permission_groups::Migration),
            Box::new(m004_create_table_permissions::Migration),
            Box::new(m005_create_table_role_groups::Migration),
            Box::new(m006_create_table_roles::Migration),
            Box::new(m007_create_table_user_groups::Migration),
            Box::new(m008_create_table_users::Migration),
            Box::new(m009_create_table_relation_permission_groups_departments::Migration),
            Box::new(m010_create_table_relation_permission_groups_roles::Migration),
            Box::new(m011_create_table_relation_permission_groups_user_groups::Migration),
            Box::new(m012_create_table_relation_permission_groups_users::Migration),
            Box::new(m013_create_table_relation_permissions_departments::Migration),
            Box::new(m014_create_table_relation_permissions_permission_groups::Migration),
            Box::new(m015_create_table_relation_permissions_roles::Migration),
            Box::new(m016_create_table_relation_permissions_user_groups::Migration),
            Box::new(m017_create_table_relation_permissions_users::Migration),
            Box::new(m018_create_table_relation_role_groups_departments::Migration),
            Box::new(m019_create_table_relation_role_groups_user_groups::Migration),
            Box::new(m020_create_table_relation_role_groups_users::Migration),
            Box::new(m021_create_table_relation_roles_departments::Migration),
            Box::new(m022_create_table_relation_roles_role_groups::Migration),
            Box::new(m023_create_table_relation_roles_user_groups::Migration),
            Box::new(m024_create_table_relation_roles_users::Migration),
            Box::new(m025_create_table_relation_users_departments::Migration),
            Box::new(m026_create_table_relation_users_user_groups::Migration),
            Box::new(m027_create_table_auth_tokens::Migration),
            Box::new(m028_create_table_third_users::Migration),
            Box::new(m029_create_table_applications::Migration),
            Box::new(m030_create_table_menus::Migration),
            Box::new(m031_create_table_codes::Migration),
            Box::new(m099_seeding_data_init::Migration),
            Box::new(m100_create_table_account_books::Migration),
            Box::new(m101_create_table_categories::Migration),
            Box::new(m102_create_table_accounts::Migration),
            Box::new(m103_create_table_relation_account_books_accounts::Migration),
            Box::new(m104_create_table_tags::Migration),
            Box::new(m105_create_table_budgets::Migration),
            Box::new(m106_create_table_transactions::Migration),
            Box::new(m107_create_table_relation_transactions_tags::Migration),
            Box::new(m108_create_table_collaborations::Migration),
        ]
    }
}
