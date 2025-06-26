use entity::departments;
use sea_orm::prelude::*;
use sea_orm::{EntityTrait, prelude::Uuid};

use crate::result::AppResult;

use super::DepartmentService;

#[derive(Debug)]
pub struct DeleteDepartmentsParams(pub Vec<Uuid>);

impl DepartmentService {
    pub async fn delete_departments(&self, params: DeleteDepartmentsParams) -> AppResult<()> {
        departments::Entity::delete_many()
            .filter(departments::Column::Id.is_in(params.0))
            .exec(&self.conn)
            .await?;

        Ok(())
    }
}
