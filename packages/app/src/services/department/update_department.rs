use entity::departments;
use sea_orm::{ActiveValue::Set, IntoActiveModel, prelude::*};

use crate::{error::AppException, result::AppResult};

use super::DepartmentService;

#[derive(Debug, Default)]
pub struct UpdateDepartmentParams {
    pub id: Uuid,
    pub name: Option<String>,
    pub parent_id: Option<Uuid>,
    pub description: Option<String>,
}

impl DepartmentService {
    pub async fn update_department(&self, params: UpdateDepartmentParams) -> AppResult<()> {
        let UpdateDepartmentParams {
            id,
            name,
            parent_id,
            description,
        } = params;

        if let Some(parent_id) = parent_id {
            let ancestors = self.query_department_ancestors(parent_id).await?;
            if ancestors.iter().find(|x| x.id == id).is_some() {
                // new parent is a child of current department
                return Err(AppException::DepartmentCircleDetected.into());
            }
        }

        let model = departments::Entity::find_by_id(id).one(&self.0).await?;
        let Some(model) = model else {
            return Err(AppException::DepartmentNotFound.into());
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
