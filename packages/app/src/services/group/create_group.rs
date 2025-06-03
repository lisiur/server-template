use entity::groups;
use sea_orm::{ActiveValue, EntityTrait, prelude::Uuid};

use crate::result::AppResult;

use super::GroupService;

#[derive(Debug, Default)]
pub struct CreateGroupParams {
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub description: Option<String>,
}

impl GroupService {
    pub async fn create_group(&self, params: CreateGroupParams) -> AppResult<Uuid> {
        let active_model = groups::ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
            name: ActiveValue::Set(params.name),
            parent_id: ActiveValue::Set(params.parent_id),
            description: ActiveValue::Set(params.description),
            is_deleted: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        };
        let result = groups::Entity::insert(active_model).exec(&self.0).await?;

        Ok(result.last_insert_id)
    }
}
