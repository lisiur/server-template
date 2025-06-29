use entity::role_groups;

use crate::impl_service;

pub mod query_role_groups;

impl_service!(RoleGroupService, role_groups::Entity);
