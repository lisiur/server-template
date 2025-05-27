use entity::users;
use sea_orm::{ActiveValue, EntityTrait, prelude::Uuid};

use crate::{models::user::Gender, result::AppResult};

use super::UserService;

#[derive(Debug, Default)]
pub struct CreateUserParams {
    pub account: String,
    pub password: String,
    pub real_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub gender: Option<Gender>,
}

impl UserService {
    pub async fn create_user(&self, params: CreateUserParams) -> AppResult<Uuid> {
        let user_active_model = users::ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
            account: ActiveValue::Set(params.account),
            nickname: ActiveValue::NotSet,
            real_name: ActiveValue::NotSet,
            phone: ActiveValue::NotSet,
            email: ActiveValue::NotSet,
            email_verified: ActiveValue::NotSet,
            avatar_url: ActiveValue::NotSet,
            gender: ActiveValue::Set(Gender::Unknown.to_string()),
            birthday: ActiveValue::NotSet,
            bio: ActiveValue::NotSet,
            password_digest: ActiveValue::NotSet,
            last_login: ActiveValue::NotSet,
            failed_login_attempts: ActiveValue::NotSet,
            is_deleted: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        };
        let result = users::Entity::insert(user_active_model).exec(&self.0).await?;

        Ok(result.last_insert_id)
    }
}
