use shared::utils::verify_password;
use uuid::Uuid;

use crate::{
    error::AppException,
    models::user::User,
    result::AppResult,
    services::{
        auth::AuthService,
        auth_token::{AuthTokenService, create_auth_token::CreateSessionTokenParams},
        department::DepartmentService,
        group::GroupService,
        role::RoleService,
        user::UserService,
    },
};

pub struct LoginParams {
    pub account: String,
    pub password: String,
    pub ip: Option<String>,
    pub platform: Option<String>,
    pub agent: Option<String>,
}

impl AuthService {
    pub async fn login(&self, params: LoginParams) -> AppResult<(Uuid, User)> {
        let user_service = UserService::new(self.0.clone());
        let auth_service = AuthService::new(self.0.clone());
        let role_service = RoleService::new(self.0.clone());
        let group_service = GroupService::new(self.0.clone());
        let department_service = DepartmentService::new(self.0.clone());
        let auth_token_service = AuthTokenService::new(self.0.clone());

        let user = user_service.query_user_by_account(&params.account).await?;
        let password_digest = user.password_digest.as_deref().unwrap_or_default();
        let password_valid = verify_password(&params.password, &password_digest);

        if !password_valid {
            return Err(AppException::AuthenticationFailed.into());
        }

        let permissions = auth_service.query_user_permissions(user.id).await?;
        let roles = role_service.query_roles_by_user_id(user.id).await?;
        let groups = group_service.query_groups_by_user_id(user.id).await?;
        let departments = department_service
            .query_departments_by_user_id(user.id)
            .await?;

        let session_id = auth_token_service
            .create_session_token(CreateSessionTokenParams {
                ip: params.ip,
                platform: params.platform,
                agent: params.agent,
                expired_at: None,
                user_id: user.id,
                permissions: permissions.into_iter().map(|x| x.code).collect(),
                roles: roles.into_iter().map(|x| x.id).collect(),
                groups: groups.into_iter().map(|x| x.id).collect(),
                departments: departments.into_iter().map(|x| x.id).collect(),
            })
            .await?;

        Ok((session_id, user))
    }
}
