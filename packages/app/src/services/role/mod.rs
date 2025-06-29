use entity::roles;

use crate::impl_service;

pub mod create_role;
pub mod delete_roles;
pub mod query_roles;
pub mod update_role;

impl_service!(RoleService, roles::Entity);
