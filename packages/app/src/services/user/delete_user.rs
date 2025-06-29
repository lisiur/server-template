use entity::users;
use sea_orm::prelude::Uuid;
use sea_orm::prelude::*;

use crate::result::AppResult;

use super::UserService;

#[derive(Debug)]
pub struct DeleteUsersParams(pub Vec<Uuid>);

impl UserService {
    pub async fn delete_users(&self, params: DeleteUsersParams) -> AppResult<()> {
        self.crud
            .delete_many(users::Column::Id.is_in(params.0))
            .await?;

        Ok(())
    }
}
