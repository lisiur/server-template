use sea_orm::DatabaseConnection;

use crate::impl_service;
pub mod create_group;
pub mod delete_groups;
pub mod query_groups;
pub mod update_group;

impl_service!(GroupService, DatabaseConnection);
