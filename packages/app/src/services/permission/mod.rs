use entity::permissions;

use crate::impl_service;
pub mod create_permission;
pub mod delete_permissions;
pub mod query_permissions;
pub mod update_permission;

impl_service!(PermissionService, permissions::Entity);
