use entity::permissions;
use entity::prelude::Permissions;
use sea_orm::{ActiveValue::Set, EntityTrait};
use uuid::Uuid;

use crate::result::AppResult;

use super::PermissionService;

#[derive(Default)]
pub struct CreatePermissionParams {
    pub code: String,
    pub kind: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
}

impl PermissionService {
    pub async fn create_permission(&self, params: CreatePermissionParams) -> AppResult<Uuid> {
        let permission_active_model = permissions::ActiveModel {
            id: Set(Uuid::new_v4()),
            code: Set(params.code),
            kind: Set(params.kind),
            description: Set(params.description),
            parent_id: Set(params.parent_id),
            ..Default::default()
        };

        let result = Permissions::insert(permission_active_model)
            .exec(&self.0)
            .await?;

        Ok(result.last_insert_id)
    }
}
