//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.10

pub mod prelude;

pub mod account_books;
pub mod accounts;
pub mod applications;
pub mod auth_tokens;
pub mod budgets;
pub mod categories;
pub mod codes;
pub mod collaborations;
pub mod departments;
pub mod menus;
pub mod permission_groups;
pub mod permissions;
pub mod relation_account_books_accounts;
pub mod relation_permission_groups_departments;
pub mod relation_permission_groups_roles;
pub mod relation_permission_groups_user_groups;
pub mod relation_permission_groups_users;
pub mod relation_permissions_departments;
pub mod relation_permissions_permission_groups;
pub mod relation_permissions_roles;
pub mod relation_permissions_user_groups;
pub mod relation_permissions_users;
pub mod relation_role_groups_departments;
pub mod relation_role_groups_user_groups;
pub mod relation_role_groups_users;
pub mod relation_roles_departments;
pub mod relation_roles_role_groups;
pub mod relation_roles_user_groups;
pub mod relation_roles_users;
pub mod relation_transactions_tags;
pub mod relation_users_departments;
pub mod relation_users_user_groups;
pub mod role_groups;
pub mod roles;
pub mod settings;
pub mod tags;
pub mod third_users;
pub mod transactions;
pub mod upload_chunks;
pub mod uploads;
pub mod user_groups;
pub mod users;
