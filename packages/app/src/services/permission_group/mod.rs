use entity::permission_groups;

use crate::impl_service;

pub mod query_permission_groups;
pub mod update_permission_group;

impl_service!(PermissionGroupService, permission_groups::Entity);
