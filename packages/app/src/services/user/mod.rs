use sea_orm::DatabaseConnection;

use crate::impl_service;
pub mod create_user;
pub mod delete_users;
pub mod query_users;

impl_service!(UserService, DatabaseConnection);
