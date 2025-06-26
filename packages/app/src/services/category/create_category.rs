use entity::categories;
use sea_orm::{
    ActiveValue::{NotSet, Set},
    EntityTrait,
    prelude::Uuid,
};
use shared::enums::CategoryType;

use crate::{result::AppResult, services::category::CategoryService};

#[derive(Debug)]
pub struct CreateCategoryParams {
    pub account_book_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub category_type: CategoryType,
    pub description: Option<String>,
}

impl CategoryService {
    pub async fn create_group(&self, params: CreateCategoryParams) -> AppResult<Uuid> {
        let active_model = categories::ActiveModel {
            id: Set(Uuid::new_v4()),
            account_book_id: Set(params.account_book_id),
            name: Set(params.name),
            parent_id: Set(params.parent_id),
            category_type: Set(params.category_type.to_string()),
            is_deleted: NotSet,
            created_at: NotSet,
            updated_at: NotSet,
        };
        let result = categories::Entity::insert(active_model)
            .exec(&self.conn)
            .await?;

        Ok(result.last_insert_id)
    }
}
