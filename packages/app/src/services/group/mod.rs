use sea_orm::DatabaseConnection;
pub mod create_group;
pub mod query_groups;

pub struct GroupService(DatabaseConnection);

impl GroupService {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self(conn)
    }
}
