use entity::user_groups;
use sea_orm::prelude::*;
use sea_orm::{EntityTrait, prelude::Uuid};

use crate::result::AppResult;

use super::UserGroupService;

#[derive(Debug)]
pub struct DeleteGroupsParams(pub Vec<Uuid>);

impl UserGroupService {
    pub async fn delete_groups(&self, params: DeleteGroupsParams) -> AppResult<()> {
        user_groups::Entity::delete_many()
            .filter(user_groups::Column::Id.is_in(params.0))
            .exec(&self.0)
            .await?;

        Ok(())
    }
}
