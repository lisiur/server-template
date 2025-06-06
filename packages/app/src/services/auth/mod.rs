use sea_orm::DatabaseConnection;
pub mod assign_permissions;
pub mod query_permissions;

pub struct AuthService(DatabaseConnection);

impl AuthService {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self(conn)
    }
}
