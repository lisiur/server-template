use sea_orm::DatabaseConnection;
pub mod create_group;
pub mod delete_groups;
pub mod query_groups;
pub mod update_group;

pub struct GroupService(DatabaseConnection);

impl GroupService {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self(conn)
    }
}
