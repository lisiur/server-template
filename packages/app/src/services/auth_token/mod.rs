use sea_orm::DatabaseConnection;
pub mod create_auth_token;
pub mod delete_auth_token;
pub mod query_auth_tokens;

pub struct AuthTokenService(DatabaseConnection);

impl AuthTokenService {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self(conn)
    }
}
