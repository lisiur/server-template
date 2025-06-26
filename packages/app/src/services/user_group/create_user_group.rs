use entity::user_groups;
use sea_orm::{ActiveValue, prelude::Uuid};

use crate::result::AppResult;

use super::UserGroupService;

#[derive(Debug, Default)]
pub struct CreateGroupParams {
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub description: Option<String>,
}

impl From<CreateGroupParams> for user_groups::ActiveModel {
    fn from(params: CreateGroupParams) -> Self {
        Self {
            id: ActiveValue::Set(Uuid::new_v4()),
            name: ActiveValue::Set(params.name),
            parent_id: ActiveValue::Set(params.parent_id),
            description: ActiveValue::Set(params.description),
            is_deleted: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        }
    }
}

impl UserGroupService {
    pub async fn create_user_group(&self, params: CreateGroupParams) -> AppResult<Uuid> {
        let result = self.crud.create(params).await?;

        Ok(result.id)
    }
}
