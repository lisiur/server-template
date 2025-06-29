use std::{path::PathBuf, sync::Arc};

use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;

use crate::result::AppResult;

#[derive(Clone)]
pub struct App {
    #[allow(dead_code)]
    pub conn: DatabaseConnection,
    pub upload_dir: Arc<PathBuf>,
}

impl App {
    pub async fn init(conn: DatabaseConnection, upload_dir: PathBuf) -> AppResult<Self> {
        Migrator::up(&conn, None).await?;

        let app = Self {
            conn,
            upload_dir: Arc::new(upload_dir),
        };

        Ok(app)
    }
}
