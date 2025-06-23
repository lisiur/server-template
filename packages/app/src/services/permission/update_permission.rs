use entity::permissions;
use sea_orm::{ActiveValue::Set, IntoActiveModel, prelude::*};

use crate::{error::AppException, result::AppResult};

use super::PermissionService;

#[derive(Debug, Default)]
pub struct UpdatePermissionParams {
    pub id: Uuid,
    pub code: Option<String>,
    pub description: Option<String>,
}

impl PermissionService {
    pub async fn update_permission(&self, params: UpdatePermissionParams) -> AppResult<()> {
        let UpdatePermissionParams {
            id,
            code,
            description,
        } = params;

        let model = permissions::Entity::find_by_id(id).one(&self.0).await?;
        let Some(model) = model else {
            return Err(AppException::PermissionNotFound.into());
        };
        let mut active_model = model.into_active_model();

        if let Some(code) = code {
            active_model.code = Set(code);
        }

        if let Some(description) = description {
            active_model.description = Set(Some(description));
        }

        active_model.update(&self.0).await?;

        Ok(())
    }
}
