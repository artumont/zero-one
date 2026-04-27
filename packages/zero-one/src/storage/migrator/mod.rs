mod schemas;

use include_dir::{Dir, include_dir};
use rusqlite::Connection;
use std::{
    collections::{BTreeMap, BTreeSet},
    io,
    path::Path,
};

use crate::storage::migrator::{schemas::DbMigration, schemas::Metadata};

static MIGRATIONS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/migrations");

/// The `Migrator` struct is responsible for managing database migrations. It maintains a connection to the database and a map of available migrations, allowing it to determine which migrations need to be applied and execute them in the correct order based on their dependencies. The migrator ensures that the necessary table for tracking applied migrations exists in the database and provides functionality to run the migrations asynchronously.
pub struct Migrator {
    conn: Connection,
    migration_map: BTreeMap<String, DbMigration>,
}

impl Migrator {
    /// Creates a new `Migrator` instance by connecting to the database at the specified path and building a map of available migrations from the embedded migrations directory. It also ensures that the necessary table for tracking applied migrations exists in the database.
    pub fn new(db_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let conn = Connection::open(db_path)?;
        let migration_map = Self::build_migration_map()?;
        Self::ensure_migrations_table(&conn)?;
        Ok(Self {
            conn,
            migration_map,
        })
    }

    /// Runs the database migrations by determining which migrations need to be applied based on the current state of the database and executing them in the correct order.
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let execution_chain = self.build_execution_chain()?;
        if execution_chain.is_empty() {
            log::info!("No new migrations to apply. Database is up to date.");
            return Ok(());
        }
        for migration in execution_chain {
            match migration.upgrade(&self.conn).await {
                Ok(_) => {
                    self.conn.execute(
                        "INSERT INTO __z1_migrations (checksum) VALUES (?1)",
                        [&migration.metadata.checksum],
                    )?;
                    log::info!(
                        "Successfully applied migration: {}",
                        migration.metadata.name
                    );
                }
                Err(e) => {
                    migration.downgrade(&self.conn).await.ok();
                    log::error!(
                        "Failed to apply migration: {}. Error: {}",
                        migration.metadata.name,
                        e
                    );
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    /// Ensures that the `__z1_migrations` table exists in the database to track applied migrations. If the table does not exist, it will be created.
    fn ensure_migrations_table(
        conn: &rusqlite::Connection,
    ) -> Result<(), Box<dyn std::error::Error>> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS __z1_migrations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                checksum TEXT NOT NULL UNIQUE,
                applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;
        Ok(())
    }

    /// Builds the execution chain of migrations that need to be applied by comparing the set of already executed migrations in the database with the available migrations in the migration map and their dependencies. The resulting execution chain is ordered to ensure that all dependencies are applied before the migrations that depend on them.
    fn build_execution_chain(&self) -> Result<Vec<&DbMigration>, Box<dyn std::error::Error>> {
        let mut executed_migrations: BTreeSet<String> = BTreeSet::new();
        let mut stmt = self.conn.prepare("SELECT checksum FROM __z1_migrations")?;
        let rows = stmt.query_map([], |row| row.get(0))?;
        for row in rows {
            let checksum: String = row?;
            executed_migrations.insert(checksum);
        }

        // Build the execution chain by checking each migration against the set of already executed migrations and their dependencies
        let mut execution_chain: Vec<&DbMigration> = Vec::new();
        for (migration_checksum, migration) in &self.migration_map {
            if executed_migrations.contains(migration_checksum) {
                log::debug!("Skipping already applied migration: {}", migration_checksum);
                continue;
            }
            // Check dependencies for the migration and add them to the execution chain if they haven't been applied yet
            for dependency in &migration.metadata.depends_on {
                if !executed_migrations.contains(dependency) {
                    log::debug!(
                        "Migration {} depends on {}, which has not been applied yet. Adding to execution chain.",
                        migration_checksum,
                        dependency
                    );
                    // Recursively add dependencies to the execution chain before adding the current migration
                    execution_chain.push(self.migration_map.get(dependency).ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::NotFound,
                            format!("Missing dependency migration with checksum: {}", dependency),
                        )
                    })?);
                }
                log::debug!(
                    "Dependency {} for migration {} has already been applied. Skipping.",
                    dependency,
                    migration_checksum
                );
            }
            log::debug!(
                "Adding migration to execution chain: {}",
                migration_checksum
            );
            execution_chain.push(migration);
        }
        Ok(execution_chain)
    }

    /// Builds a map of available migrations by reading the embedded migrations directory and parsing the metadata and SQL files for each migration. The map is keyed by the checksum of the migration, allowing for easy lookup when determining which migrations need to be applied.
    fn build_migration_map() -> Result<BTreeMap<String, DbMigration>, Box<dyn std::error::Error>> {
        let mut migrations: BTreeMap<String, DbMigration> = BTreeMap::new();
        for folder in MIGRATIONS_DIR.dirs() {
            let metadata_path = format!("{}/metadata.json", folder.path().display());
            let up_path = format!("{}/up.sql", folder.path().display());
            let down_path = format!("{}/down.sql", folder.path().display());
            let metadata_file = folder
                .get_file("metadata.json")
                .or_else(|| MIGRATIONS_DIR.get_file(metadata_path.as_str()))
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::NotFound,
                        format!(
                            "Missing metadata.json in migration folder: {}",
                            folder.path().display()
                        ),
                    )
                })?;
            let metadata_content = metadata_file.contents_utf8().ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Invalid UTF-8 in metadata.json for migration: {}",
                        folder.path().display()
                    ),
                )
            })?;
            let metadata: Metadata = serde_json::from_str(metadata_content)?;
            let up_file = folder
                .get_file("up.sql")
                .or_else(|| MIGRATIONS_DIR.get_file(up_path.as_str()))
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::NotFound,
                        format!(
                            "Missing up.sql in migration folder: {}",
                            folder.path().display()
                        ),
                    )
                })?;
            let down_file = folder
                .get_file("down.sql")
                .or_else(|| MIGRATIONS_DIR.get_file(down_path.as_str()))
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::NotFound,
                        format!(
                            "Missing down.sql in migration folder: {}",
                            folder.path().display()
                        ),
                    )
                })?;
            let up_sql = up_file.contents_utf8().ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Invalid UTF-8 in up.sql for migration: {}",
                        folder.path().display()
                    ),
                )
            })?;
            let down_sql = down_file.contents_utf8().ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Invalid UTF-8 in down.sql for migration: {}",
                        folder.path().display()
                    ),
                )
            })?;
            migrations.insert(
                metadata.checksum.clone(),
                DbMigration::new(metadata, up_sql.to_string(), down_sql.to_string()),
            );
        }
        Ok(migrations)
    }
}
