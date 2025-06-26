use entity::categories;
use sea_orm::DatabaseConnection;

use crate::impl_service;
pub mod create_category;
pub mod delete_categories;
pub mod query_categories;
pub mod update_category;

impl_service!(CategoryService, DatabaseConnection, categories::Entity);
