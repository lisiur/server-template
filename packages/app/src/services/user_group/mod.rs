use sea_orm::DatabaseConnection;

use crate::impl_service;
pub mod create_user_group;
pub mod delete_user_groups;
pub mod query_user_groups;
pub mod update_user_group;

impl_service!(UserGroupService, DatabaseConnection);
