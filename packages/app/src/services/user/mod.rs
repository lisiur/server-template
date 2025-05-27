use sea_orm::DatabaseConnection;
pub mod create_user;
pub mod query_users;

pub struct UserService(DatabaseConnection);

impl UserService {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self(conn)
    }
}
