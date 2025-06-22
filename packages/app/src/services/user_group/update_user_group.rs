use entity::user_groups;
use sea_orm::{ActiveValue::Set, IntoActiveModel, prelude::*};

use crate::{error::AppException, result::AppResult};

use super::UserGroupService;

#[derive(Debug, Default)]
pub struct UpdateGroupParams {
    pub id: Uuid,
    pub name: Option<String>,
    pub parent_id: Option<Uuid>,
    pub description: Option<String>,
}

impl UserGroupService {
    pub async fn update_group(&self, params: UpdateGroupParams) -> AppResult<()> {
        let UpdateGroupParams {
            id,
            name,
            parent_id,
            description,
        } = params;

        if let Some(parent_id) = parent_id {
            let group_chains = self.query_user_group_ancestors(parent_id).await?;
            if group_chains.iter().find(|x| x.id == id).is_some() {
                // new parent is a child of current group
                return Err(AppException::GroupCircleDetected.into());
            }
        }

        let model = user_groups::Entity::find_by_id(id).one(&self.0).await?;
        let Some(model) = model else {
            return Err(AppException::UserGroupNotFound.into());
        };
        let mut active_model = model.into_active_model();

        if let Some(name) = name {
            active_model.name = Set(name);
        }

        if let Some(parent_id) = parent_id {
            active_model.parent_id = Set(Some(parent_id));
        } else {
            active_model.parent_id = Set(None);
        }

        if let Some(description) = description {
            active_model.description = Set(Some(description));
        }

        active_model.update(&self.0).await?;

        Ok(())
    }
}
