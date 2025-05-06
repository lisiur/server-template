use entity::user;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait, prelude::Uuid};

use crate::result::AppResult;

pub struct UserService(DatabaseConnection);

impl UserService {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self(conn)
    }

    pub async fn create_user(&self) -> AppResult<Uuid> {
        let user = user::ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
            account: ActiveValue::Set("admin".to_string()),
            nickname: ActiveValue::NotSet,
            realname: ActiveValue::NotSet,
            phone: ActiveValue::NotSet,
            email: ActiveValue::NotSet,
            email_verified: ActiveValue::NotSet,
            avatar_url: ActiveValue::NotSet,
            gender: ActiveValue::Set(user::Gender::Male),
            birthday: ActiveValue::NotSet,
            bio: ActiveValue::NotSet,
            password_digest: ActiveValue::NotSet,
            last_login: ActiveValue::NotSet,
            failed_login_attempts: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        };
        let user = user::Entity::insert(user).exec(&self.0).await?;

        Ok(user.last_insert_id)
    }
}
