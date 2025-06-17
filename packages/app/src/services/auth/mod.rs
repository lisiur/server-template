use sea_orm::DatabaseConnection;

use crate::impl_service;
pub mod assign_permissions;
pub mod login;
pub mod logout;
pub mod query_permissions;

impl_service!(AuthService, DatabaseConnection);
