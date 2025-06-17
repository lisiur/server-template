use entity::roles;
use sea_orm::{ActiveValue::Set, EntityTrait, prelude::Uuid};

use crate::result::AppResult;

use super::RoleService;

#[derive(Debug, Default)]
pub struct CreateRoleParams {
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub description: Option<String>,
}

impl RoleService {
    pub async fn create_role(&self, params: CreateRoleParams) -> AppResult<Uuid> {
        let active_model = roles::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(params.name),
            description: Set(params.description),
            parent_id: Set(params.parent_id),
            built_in: Set(false),
            ..Default::default()
        };
        let result = roles::Entity::insert(active_model).exec(&self.0).await?;

        Ok(result.last_insert_id)
    }
}
