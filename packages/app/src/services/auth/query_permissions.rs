use crate::{models::permission::Permission, result::AppResult};

use super::AuthService;

impl AuthService {
    pub async fn query_user_permissions() -> AppResult<Vec<Permission>> {
        todo!()
    }

    pub async fn query_role_permissions() -> AppResult<Vec<Permission>> {
        todo!()
    }

    pub async fn query_group_permissions() -> AppResult<Vec<Permission>> {
        todo!()
    }
}
