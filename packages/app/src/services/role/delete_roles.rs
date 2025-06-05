use entity::roles;
use sea_orm::prelude::*;
use sea_orm::{EntityTrait, prelude::Uuid};

use crate::result::AppResult;

use super::RoleService;

#[derive(Debug)]
pub struct DeleteRolesParams(pub Vec<Uuid>);

impl RoleService {
    pub async fn delete_roles(&self, params: DeleteRolesParams) -> AppResult<()> {
        roles::Entity::delete_many()
            .filter(roles::Column::Id.is_in(params.0))
            .exec(&self.0)
            .await?;

        Ok(())
    }
}
