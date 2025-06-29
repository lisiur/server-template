use entity::user_groups;

use crate::impl_service;
pub mod create_user_group;
pub mod delete_user_group;
pub mod query_user_group;
pub mod update_user_group;

impl_service!(UserGroupService, user_groups::Entity);
