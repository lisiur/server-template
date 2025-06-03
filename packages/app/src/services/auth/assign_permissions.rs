use entity::{prelude::*, relation_permissions_users};
use sea_orm::{ActiveValue, EntityTrait};
use uuid::Uuid;

use crate::result::AppResult;

use super::AuthService;

pub struct AssignUserPermissionsParams {
    pub user_id: Uuid,
    pub permission_id_list: Vec<Uuid>,
}

impl AuthService {
    pub async fn assign_user_permissions(
        &self,
        params: AssignUserPermissionsParams,
    ) -> AppResult<()> {
        RelationPermissionsUsers::insert_many(params.permission_id_list.iter().map(
            |permission_id| relation_permissions_users::ActiveModel {
                user_id: ActiveValue::Set(params.user_id),
                permission_id: ActiveValue::Set(*permission_id),
                ..Default::default()
            },
        ))
        .exec(&self.0)
        .await?;

        Ok(())
    }
}
