use entity::roles;
use sea_orm::DatabaseConnection;

use crate::impl_service;

pub mod create_role;
pub mod delete_roles;
pub mod query_roles;
pub mod update_role;

impl_service!(RoleService, DatabaseConnection, roles::Entity);
