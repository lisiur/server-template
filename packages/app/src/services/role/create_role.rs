use entity::roles;
use sea_orm::{ActiveValue, EntityTrait, prelude::Uuid};

use crate::result::AppResult;

use super::RoleService;

#[derive(Debug, Default)]
pub struct CreateRoleParams {
    pub name: String,
    pub description: Option<String>,
}

impl RoleService {
    pub async fn create_role(&self, params: CreateRoleParams) -> AppResult<Uuid> {
        let active_model = roles::ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
            name: ActiveValue::Set(params.name),
            description: ActiveValue::Set(params.description),
            is_deleted: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        };
        let result = roles::Entity::insert(active_model).exec(&self.0).await?;

        Ok(result.last_insert_id)
    }
}
