use entity::user_groups;
use sea_orm::DatabaseConnection;

use crate::impl_service;
pub mod create_user_group;
pub mod delete_user_group;
pub mod query_user_group;
pub mod update_user_group;

impl_service!(UserGroupService, DatabaseConnection, user_groups::Entity);
