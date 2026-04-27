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
    /// Creates a new `DBManager` instance by connecting to the database at the specified path.
    async fn new(db_path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let migrator = migrator::Migrator::new(&db_path)?;
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

    pub async fn run_migrations(&self) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Running database migrations...");
        self.migrator.run().await?;
        log::info!("Database migrations completed successfully.");
        Ok(())
    }
}

/// Asynchronously retrieves a `DBManager` instance connected to the application's database. This function ensures that the database file exists and is ready for use.
pub async fn get_db_manager() -> Result<DBManager, Box<dyn std::error::Error>> {
    let db_path = ensure_app_database()?;
    let manager = DBManager::new(db_path).await?;
    manager.run_migrations().await?;
    Ok(manager)
}
