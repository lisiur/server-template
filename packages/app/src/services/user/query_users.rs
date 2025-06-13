use entity::users;
use sea_orm::prelude::*;

use crate::{
    error::AppException,
    models::user::User,
    result::AppResult,
    utils::query::{FilterAtom, FilterCondition, PageableQuery, SelectQuery},
};

use super::UserService;

impl UserService {
    pub async fn query_user_by_id(&self, id: Uuid) -> AppResult<User> {
        let user = users::Entity::find_by_id(id).one(&self.0).await?;

        let Some(user) = user else {
            return Err(AppException::UserNotFound.into());
        };

        Ok(user.into())
    }

    pub async fn query_user_by_account(&self, account: &str) -> AppResult<User> {
        let user = users::Entity::find()
            .filter(users::Column::Account.eq(account))
            .one(&self.0)
            .await?;

        let Some(user) = user else {
            return Err(AppException::UserNotFound.into());
        };

        Ok(user.into())
    }

    pub async fn query_users_list(&self) -> AppResult<Vec<User>> {
        let users = users::Entity::find().all(&self.0).await?;

        let users = users.into_iter().map(User::from).collect();

        Ok(users)
    }

    pub async fn query_users_by_page<T: PageableQuery<FilterUsersParams>>(
        &self,
        params: T,
    ) -> AppResult<(Vec<User>, i64)> {
        let mut select_query = SelectQuery::default().with_cursor(params.cursor());
        let filter = params.into_filter();
        if let Some(ref name) = filter.account {
            if !name.is_empty() {
                select_query.add_atom_filter(FilterAtom {
                    field: users::Column::Account.as_str().to_string(),
                    condition: FilterCondition::Like(format!("%{name}%")),
                });
            }
        }
        let (users, count) = select_query
            .all_with_count::<users::Model>(users::Entity, &self.0)
            .await?;

        let users = users.into_iter().map(User::from).collect();

        Ok((users, count))
    }
}

pub struct FilterUsersParams {
    pub account: Option<String>,
}
