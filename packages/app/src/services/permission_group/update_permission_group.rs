use sea_orm::prelude::*;

use crate::result::AppResult;

use super::PermissionGroupService;

#[derive(Debug, Default)]
pub struct UpdatePermissionGroupParams {
    pub id: Uuid,
    pub name: Option<String>,
    pub parent_id: Option<Uuid>,
    pub description: Option<String>,
}

impl PermissionGroupService {
    pub async fn update_permission_group(
        &self,
        _params: UpdatePermissionGroupParams,
    ) -> AppResult<()> {
        todo!()
    }
}
