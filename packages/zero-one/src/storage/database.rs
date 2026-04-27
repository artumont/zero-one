use std::path::PathBuf;

use crate::{storage::migrator, utils::ensure_app_database};

fn file_path_to_connection_url(path: &str) -> String {
    if cfg!(target_os = "windows") {
        format!("sqlite:///{}", path.replace("\\", "/"))
    } else {
        format!("sqlite://{}", path)
    }
}

pub struct DBManager {
    pub db: toasty::Db,
    pub migrator: migrator::Migrator,
}

impl DBManager {
    /// Creates a new `DBManager` instance. Migrations are run first (before the toasty
    /// connection is opened) so the schema is always up to date before any ORM code runs
    /// and to avoid SQLite locking contention between two simultaneous connections.
    async fn new(db_path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let mut migrator = migrator::Migrator::new(&db_path)?;
        log::info!("Running database migrations...");
        migrator.run().await?;
        log::info!("Database migrations completed successfully.");

        let converted_db_path =
            file_path_to_connection_url(db_path.to_str().expect("Invalid database path"));
        let db = toasty::Db::builder()
            .models(crate::storage_models!())
            .connect(&converted_db_path)
            .await
            .expect("Failed to connect to database");
        log::debug!("Connected to database at: {}", converted_db_path);
        Ok(DBManager { db, migrator })
    }
}

/// Asynchronously retrieves a `DBManager` instance connected to the application's database. This function ensures that the database file exists and is ready for use.
pub async fn get_db_manager() -> Result<DBManager, Box<dyn std::error::Error>> {
    let db_path = ensure_app_database()?;
    DBManager::new(db_path).await
}
