use entity::roles;
use sea_orm::{ActiveValue::Set, IntoActiveModel, prelude::*};

use crate::{error::AppException, result::AppResult};

use super::RoleService;

#[derive(Debug, Default)]
pub struct UpdateRoleParams {
    pub id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl RoleService {
    pub async fn update_role(&self, params: UpdateRoleParams) -> AppResult<()> {
        let UpdateRoleParams {
            id,
            name,
            description,
        } = params;

        let model = roles::Entity::find_by_id(id).one(&self.0).await?;
        let Some(model) = model else {
            return Err(AppException::RoleNotFound.into());
        };
        let mut active_model = model.into_active_model();

        if let Some(name) = name {
            active_model.name = Set(name);
        }

        if let Some(description) = description {
            active_model.description = Set(Some(description));
        }

        active_model.update(&self.0).await?;

        Ok(())
    }
}
