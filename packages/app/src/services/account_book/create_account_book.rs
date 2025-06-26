use entity::account_books;
use sea_orm::{
    ActiveValue::{NotSet, Set},
    EntityTrait,
    prelude::Uuid,
};
use shared::enums::CategoryType;

use crate::{result::AppResult, services::account_book::AccountBookService};

#[derive(Debug)]
pub struct CreateAccountBookParams {
    pub owner_id: Uuid,
    pub name: String,
    pub currency: String,
    pub category_type: CategoryType,
    pub description: Option<String>,
}

impl AccountBookService {
    pub async fn create_account_book(&self, params: CreateAccountBookParams) -> AppResult<Uuid> {
        let active_model = account_books::ActiveModel {
            id: Set(Uuid::new_v4()),
            owner_id: Set(params.owner_id),
            name: Set(params.name),
            currency: Set(params.currency),
            is_deleted: NotSet,
            created_at: NotSet,
            updated_at: NotSet,
        };
        let result = account_books::Entity::insert(active_model)
            .exec(&self.conn)
            .await?;

        Ok(result.last_insert_id)
    }
}
