use entity::users;
use sea_orm::EntityTrait;

use crate::{models::user::User, result::AppResult};

use super::UserService;

impl UserService {
    pub async fn query_users_list(&self) -> AppResult<Vec<User>> {
        let users = users::Entity::find().all(&self.0).await?;
        
        let users = users.into_iter().map(User::from).collect();
        
        Ok(users)
    }
}