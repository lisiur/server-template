use sea_orm::DatabaseConnection;

use crate::impl_service;

pub mod query_permission_groups;

impl_service!(PermissionGroupService, DatabaseConnection);
