use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;

use crate::result::AppResult;

pub struct App {
    #[allow(dead_code)]
    db_conn: DatabaseConnection,
}

impl App {
    pub async fn init(db_conn: DatabaseConnection) -> AppResult<Self> {
        Migrator::up(&db_conn, None).await?;

        let app = Self { db_conn };

        Ok(app)
    }
}
