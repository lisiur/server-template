use shared::enums::Gender;
use uuid::Uuid;

use crate::{
    models::user::User,
    result::AppResult,
    services::{
        auth::{AuthService, login::LoginParams},
        user::{UserService, create_user::CreateUserParams},
    },
};

pub struct RegisterParams {
    pub account: String,
    pub password: String,
    pub ip: Option<String>,
    pub platform: Option<String>,
    pub agent: Option<String>,
}

impl AuthService {
    pub async fn register(&self, params: RegisterParams) -> AppResult<(Uuid, User)> {
        let user_service = UserService::new(self.app.clone());
        user_service
            .create_user(CreateUserParams {
                account: params.account.clone(),
                password: params.password.clone(),
                real_name: None,
                phone: None,
                email: None,
                gender: Some(Gender::Unknown),
            })
            .await?;

        self.login(LoginParams {
            account: params.account,
            password: params.password,
            ip: params.ip,
            platform: params.platform,
            agent: params.agent,
        })
        .await
    }
}
