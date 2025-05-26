use sea_orm::DatabaseConnection;
pub mod create_user;

pub struct UserService(DatabaseConnection);

impl UserService {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self(conn)
    }
}
