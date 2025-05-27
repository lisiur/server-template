use entity::{permissions, users};
use sea_orm::prelude::*;
use uuid::Uuid;

use crate::{models::permission::Permission, result::AppResult};

use super::AuthService;

impl AuthService {
    pub async fn query_user_permissions(&self, user_id: Uuid) -> AppResult<Vec<Permission>> {
        let user = users::Entity::find_by_id(user_id).one(&self.0).await?;
        let Some(user) = user else {
            return Ok(vec![]);
        };

        let permissions = user.find_related(permissions::Entity).all(&self.0).await?;
        let permissions = permissions.into_iter().map(Permission::from).collect();

        Ok(permissions)
    }

    pub async fn query_role_permissions() -> AppResult<Vec<Permission>> {
        todo!()
    }

    pub async fn query_group_permissions() -> AppResult<Vec<Permission>> {
        todo!()
    }
}
