use entity::groups;
use sea_orm::prelude::*;
use sea_orm::{ActiveValue, EntityTrait, prelude::Uuid};

use crate::result::AppResult;

use super::GroupService;

#[derive(Debug, Default)]
pub struct CreateGroupParams {
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub description: Option<String>,
}

#[derive(Debug)]
pub struct DeleteGroupsParams(pub Vec<Uuid>);

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

    pub async fn delete_groups(&self, params: DeleteGroupsParams) -> AppResult<()> {
        groups::Entity::delete_many()
            .filter(groups::Column::Id.is_in(params.0))
            .exec(&self.0)
            .await?;

        Ok(())
    }
}
