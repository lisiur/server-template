use entity::users;
use sea_orm::prelude::*;
use sea_orm::{EntityTrait, prelude::Uuid};

use crate::result::AppResult;

use super::UserService;

#[derive(Debug)]
pub struct DeleteUsersParams(pub Vec<Uuid>);

impl UserService {
    pub async fn delete_users(&self, params: DeleteUsersParams) -> AppResult<()> {
        users::Entity::delete_many()
            .filter(users::Column::Id.is_in(params.0))
            .exec(&self.0)
            .await?;

        Ok(())
    }
}
