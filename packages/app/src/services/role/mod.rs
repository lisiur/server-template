use sea_orm::DatabaseConnection;

pub mod query_roles;

pub struct RoleService(DatabaseConnection);

impl RoleService {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self(conn)
    }
}
