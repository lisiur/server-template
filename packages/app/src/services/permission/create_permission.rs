use entity::prelude::Permissions;
use entity::relation_permissions_permission_groups;
use entity::{permissions, prelude::RelationPermissionsPermissionGroups};
use sea_orm::{ActiveValue::Set, EntityTrait, TransactionTrait};
use uuid::Uuid;

use crate::result::AppResult;

use super::PermissionService;

#[derive(Default)]
pub struct CreatePermissionParams {
    pub code: String,
    pub kind: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
}

impl PermissionService {
    pub async fn create_permission(&self, params: CreatePermissionParams) -> AppResult<Uuid> {
        let tx = self.conn.begin().await?;

        let permission_active_model = permissions::ActiveModel {
            id: Set(Uuid::new_v4()),
            code: Set(params.code),
            kind: Set(params.kind),
            description: Set(params.description),
            ..Default::default()
        };

        let permission_id = Permissions::insert(permission_active_model)
            .exec(&tx)
            .await?
            .last_insert_id;

        if let Some(permission_group_id) = params.parent_id {
            RelationPermissionsPermissionGroups::insert(
                relation_permissions_permission_groups::ActiveModel {
                    permission_id: Set(permission_id),
                    permission_group_id: Set(permission_group_id),
                    ..Default::default()
                },
            )
            .exec(&tx)
            .await?;
        }

        tx.commit().await?;

        Ok(permission_id)
    }
}
