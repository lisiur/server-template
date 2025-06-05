use entity::groups;
use sea_orm::prelude::*;
use sea_orm::{EntityTrait, prelude::Uuid};

use crate::result::AppResult;

use super::GroupService;

#[derive(Debug)]
pub struct DeleteGroupsParams(pub Vec<Uuid>);

impl GroupService {
    pub async fn delete_groups(&self, params: DeleteGroupsParams) -> AppResult<()> {
        groups::Entity::delete_many()
            .filter(groups::Column::Id.is_in(params.0))
            .exec(&self.0)
            .await?;

        Ok(())
    }
}
