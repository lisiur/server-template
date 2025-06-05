use sea_orm::DatabaseConnection;

pub mod create_role;
pub mod delete_roles;
pub mod query_roles;
pub mod update_role;

pub struct RoleService(DatabaseConnection);

impl RoleService {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self(conn)
    }
}
