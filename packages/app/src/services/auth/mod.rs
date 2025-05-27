use sea_orm::DatabaseConnection;
pub mod query_permissions;

pub struct AuthService(DatabaseConnection);

impl AuthService {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self(conn)
    }
}
