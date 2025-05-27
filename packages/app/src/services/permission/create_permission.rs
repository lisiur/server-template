use entity::permissions;
use sea_orm::{ActiveValue, EntityTrait};
use uuid::Uuid;

use crate::result::AppResult;

use super::PermissionService;

pub struct CreatePermissionParams {
    pub code: String,
    pub kind: String,
    pub description: Option<String>,
}

impl PermissionService {
    pub async fn create_permission(&self, params: CreatePermissionParams) -> AppResult<Uuid> {
        let permission_active_model = permissions::ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
            code: ActiveValue::Set(params.code),
            kind: ActiveValue::Set(params.kind),
            description: ActiveValue::Set(params.description),
            ..Default::default()
        };
        
        let result = permissions::Entity::insert(permission_active_model).exec(&self.0).await?;
        
        Ok(result.last_insert_id)
    }
}