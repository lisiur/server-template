use entity::categories;

use crate::impl_service;
pub mod create_category;
pub mod delete_categories;
pub mod query_categories;
pub mod update_category;

impl_service!(CategoryService, categories::Entity);
