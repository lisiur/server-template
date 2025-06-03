use entity::{prelude::*, relation_permissions_users};
use sea_orm::{ActiveValue, EntityTrait};
use uuid::Uuid;

use crate::result::AppResult;

use super::AuthService;

pub struct AssignUserPermissionParams {
    pub user_id: Uuid,
    pub permission_id: Uuid,
}

impl AuthService {
    pub async fn assign_user_permissions(
        &self,
        params: AssignUserPermissionParams,
    ) -> AppResult<()> {
        RelationPermissionsUsers::insert(relation_permissions_users::ActiveModel {
            user_id: ActiveValue::Set(params.user_id),
            permission_id: ActiveValue::Set(params.permission_id),
            ..Default::default()
        })
        .exec(&self.0)
        .await?;

        Ok(())
    }
}
