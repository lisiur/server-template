use entity::user_groups;
use sea_orm::prelude::Uuid;
use sea_orm::prelude::*;

use crate::result::AppResult;

use super::UserGroupService;

#[derive(Debug)]
pub struct DeleteGroupsParams(pub Vec<Uuid>);

impl UserGroupService {
    pub async fn delete_user_group_by_id(&self, id: Uuid) -> AppResult<()> {
        self.crud.delete_by_id(id).await?;

        Ok(())
    }

    pub async fn delete_user_groups(&self, params: DeleteGroupsParams) -> AppResult<()> {
        self.crud
            .delete_many(user_groups::Column::Id.is_in(params.0))
            .await?;

        Ok(())
    }
}
