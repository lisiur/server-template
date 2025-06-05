use entity::permissions;
use sea_orm::prelude::*;
use sea_orm::{EntityTrait, prelude::Uuid};

use crate::result::AppResult;

use super::PermissionService;

#[derive(Debug)]
pub struct DeletePermissionsParams(pub Vec<Uuid>);

impl PermissionService {
    pub async fn delete_permissions(&self, params: DeletePermissionsParams) -> AppResult<()> {
        permissions::Entity::delete_many()
            .filter(permissions::Column::Id.is_in(params.0))
            .exec(&self.0)
            .await?;

        Ok(())
    }
}
