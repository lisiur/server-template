use sea_orm::DatabaseConnection;
pub mod create_department;
pub mod delete_departments;
pub mod query_departments;
pub mod update_department;

pub struct DepartmentService(DatabaseConnection);

impl DepartmentService {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self(conn)
    }
}
