use entity::users;
use sea_orm::DatabaseConnection;

use crate::impl_service;
pub mod create_user;
pub mod delete_user;
pub mod query_user;

impl_service!(UserService, DatabaseConnection, users::Entity);
