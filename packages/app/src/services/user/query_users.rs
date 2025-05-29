use entity::users;
use sea_orm::prelude::*;

use crate::{models::user::User, result::AppResult, utils::query::SelectQuery};

use super::UserService;

impl UserService {
    pub async fn query_users_list(&self) -> AppResult<Vec<User>> {
        let users = users::Entity::find().all(&self.0).await?;

        let users = users.into_iter().map(User::from).collect();

        Ok(users)
    }

    pub async fn query_users_by_page(&self, query: SelectQuery) -> AppResult<(Vec<User>, i64)> {
        let (users, count) = query
            .all_with_count::<users::Model>(&self.0, users::Entity)
            .await?;

        let users = users.into_iter().map(User::from).collect();

        Ok((users, count))
    }
}

pub enum UserFilter {}
