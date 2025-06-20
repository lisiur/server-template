use sea_orm::DatabaseConnection;

use crate::impl_service;

pub mod query_role_groups;

impl_service!(RoleGroupService, DatabaseConnection);
