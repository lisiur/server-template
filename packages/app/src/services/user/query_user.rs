use entity::users;
use migration::{ColumnRef, IntoColumnRef};
use sea_orm::{Condition, prelude::*};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{
    error::AppException, models::user::User, result::AppResult, utils::query::PageableQuery,
};

use super::UserService;

pub struct FilterUsersParams {
    pub account: Option<String>,
}

impl From<FilterUsersParams> for Condition {
    fn from(value: FilterUsersParams) -> Self {
        Condition::all().add_option(
            value
                .account
                .map(|account| users::Column::Account.like(account)),
        )
    }
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum UserOrderField {
    Account,
}

impl From<UserOrderField> for ColumnRef {
    fn from(value: UserOrderField) -> Self {
        match value {
            UserOrderField::Account => users::Column::Account.into_column_ref(),
        }
    }
}

impl UserService {
    pub async fn query_user_by_id(&self, id: Uuid) -> AppResult<User> {
        let user = self.crud.find_by_id(id).await?;

        let Some(user) = user else {
            return Err(AppException::UserNotFound.into());
        };

        Ok(user.into())
    }

    pub async fn query_user_by_account(&self, account: &str) -> AppResult<User> {
        let user = self
            .crud
            .find_one_by_condition(users::Column::Account.eq(account))
            .await?;

        let Some(user) = user else {
            return Err(AppException::UserNotFound.into());
        };

        Ok(user.into())
    }

    pub async fn query_users_by_page(
        &self,
        params: PageableQuery<FilterUsersParams, UserOrderField>,
    ) -> AppResult<(Vec<User>, i64)> {
        let (records, total) = self.crud.find_by_condition_with_count(params).await?;

        let users = records.into_iter().map(User::from).collect();

        Ok((users, total))
    }
}
