use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;

use crate::{result::AppResult, user::UserService};

pub struct App {
    db_conn: DatabaseConnection,
}

impl App {
    pub async fn init(db_conn: DatabaseConnection) -> AppResult<Self> {
        Migrator::up(&db_conn, None).await?;

        let app = Self { db_conn };

        Ok(app)
    }
    
    pub fn user_service(&self) -> UserService {
        UserService::new(self.db_conn.clone())
    }
}
