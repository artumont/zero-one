use std::path::PathBuf;

use crate::utils::ensure_app_database;

fn file_path_to_connection_url(path: &str) -> String {
    if cfg!(target_os = "windows") {
        format!("sqlite:///{}", path.replace("\\", "/"))
    } else {
        format!("sqlite://{}", path)
    }
}

pub struct DBManager {
    pub db: toasty::Db,
}

impl DBManager {
    /// Creates a new `DBManager` instance by connecting to the database at the specified path.
    async fn new(db_path: PathBuf) -> Self {
        let converted_db_path =
            file_path_to_connection_url(db_path.to_str().expect("Invalid database path"));
        let db = toasty::Db::builder()
            .models(crate::storage_models!())
            .connect(&converted_db_path)
            .await
            .expect("Failed to connect to database");
        DBManager { db }
    }

    pub async fn execute_migrations(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Implement migration logic here, e.g., using embedded SQL files or a migration library.
        // For example:
        // let migration_sql = include_str!("migrations/001_create_tables.sql");
        // self.db.execute(migration_sql).await?;
        Ok(())
    }
}

/// Asynchronously retrieves a `DBManager` instance connected to the application's database. This function ensures that the database file exists and is ready for use.
pub async fn get_db_manager() -> Result<DBManager, Box<dyn std::error::Error>> {
    let db_path = ensure_app_database()?;
    Ok(DBManager::new(db_path).await)
}
