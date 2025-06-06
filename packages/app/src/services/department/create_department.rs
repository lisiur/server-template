use entity::departments;
use sea_orm::{ActiveValue, EntityTrait, prelude::Uuid};

use crate::result::AppResult;

use super::DepartmentService;

#[derive(Debug, Default)]
pub struct CreateDepartmentParams {
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub description: Option<String>,
}

impl DepartmentService {
    pub async fn create_department(&self, params: CreateDepartmentParams) -> AppResult<Uuid> {
        let active_model = departments::ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
            name: ActiveValue::Set(params.name),
            parent_id: ActiveValue::Set(params.parent_id),
            description: ActiveValue::Set(params.description),
            is_deleted: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        };
        let result = departments::Entity::insert(active_model)
            .exec(&self.0)
            .await?;

        Ok(result.last_insert_id)
    }
}
