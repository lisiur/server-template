use sea_orm::DatabaseConnection;

use crate::impl_service;
pub mod create_permission;
pub mod delete_permissions;
pub mod query_permissions;

impl_service!(PermissionService, DatabaseConnection);
