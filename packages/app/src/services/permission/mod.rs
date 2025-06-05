use sea_orm::DatabaseConnection;
pub mod create_permission;
pub mod delete_permissions;
pub mod query_permissions;

pub struct PermissionService(DatabaseConnection);

impl PermissionService {
    pub fn new(conn: DatabaseConnection) -> Self {
        PermissionService(conn)
    }
}
