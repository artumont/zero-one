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

/// The `Migrator` struct is responsible for managing database migrations. It maintains a connection to the database and a map of available migrations, allowing it to determine which migrations need to be applied and execute them in the correct order based on their dependencies. The migrator ensures that the necessary table for tracking applied migrations exists in the database and provides functionality to run the migrations.
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

    /// Runs the database migrations by determining which migrations need to be applied based on the current state of the database and executing them in the correct order. Each migration is wrapped in a rusqlite transaction: on success the transaction (including the tracking-table insert) is committed; on failure the transaction is automatically rolled back.
    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let execution_chain = self.build_execution_chain()?;
        if execution_chain.is_empty() {
            log::info!("No new migrations to apply. Database is up to date.");
            return Ok(());
        }

        // Collect owned migration data before entering the transaction loop to avoid
        // holding an immutable borrow on `self.migration_map` while we need `&mut self.conn`.
        let migrations_to_apply: Vec<(String, String, String)> = execution_chain
            .into_iter()
            .map(|checksum| {
                let m = &self.migration_map[&checksum];
                (checksum, m.metadata.name.clone(), m.up.clone())
            })
            .collect();

        for (checksum, name, up_sql) in migrations_to_apply {
            // Each migration runs inside its own transaction; if `execute_batch` fails the
            // transaction is dropped (auto-rolled back) without touching the DB state.
            let tx = self.conn.transaction()?;
            match tx.execute_batch(&up_sql) {
                Ok(_) => {
                    // The INSERT is intentionally inside the same transaction as the migration
                    // SQL so that both are committed atomically: if commit fails, neither the
                    // schema change nor the tracking record will be persisted.
                    tx.execute(
                        "INSERT INTO __z1_migrations (checksum) VALUES (?1)",
                        [&checksum],
                    )?;
                    tx.commit()?;
                    log::info!("Successfully applied migration: {}", name);
                }
                Err(e) => {
                    // `tx` is dropped here, which auto-rolls back the transaction.
                    log::error!(
                        "Failed to apply migration: {}. Error: {}",
                        name,
                        e
                    );
                    return Err(Box::new(e));
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

    /// Builds the ordered execution chain of migrations that need to be applied. Uses a DFS-based topological sort with cycle detection to ensure that all transitive dependencies are scheduled before their dependents, without duplicates.
    fn build_execution_chain(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut executed_migrations: BTreeSet<String> = BTreeSet::new();
        let mut stmt = self.conn.prepare("SELECT checksum FROM __z1_migrations")?;
        let rows = stmt.query_map([], |row| row.get(0))?;
        for row in rows {
            let checksum: String = row?;
            executed_migrations.insert(checksum);
        }

        let mut result: Vec<String> = Vec::new();
        let mut visiting: BTreeSet<String> = BTreeSet::new();
        let mut visited: BTreeSet<String> = BTreeSet::new();

        for checksum in self.migration_map.keys() {
            if !executed_migrations.contains(checksum) && !visited.contains(checksum) {
                self.topo_visit(
                    checksum,
                    &executed_migrations,
                    &mut visiting,
                    &mut visited,
                    &mut result,
                )?;
            }
        }
        Ok(result)
    }

    /// Recursively visits a migration and its dependencies in DFS order, appending each migration to `result` only after all its dependencies have been added (topological order). Detects cycles via the `visiting` set.
    fn topo_visit(
        &self,
        checksum: &str,
        executed: &BTreeSet<String>,
        visiting: &mut BTreeSet<String>,
        visited: &mut BTreeSet<String>,
        result: &mut Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if visited.contains(checksum) || executed.contains(checksum) {
            return Ok(());
        }
        if visiting.contains(checksum) {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Circular dependency detected for migration: {}", checksum),
            )));
        }

        let migration = self.migration_map.get(checksum).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("Missing dependency migration with checksum: {}", checksum),
            )
        })?;

        visiting.insert(checksum.to_string());
        for dep in &migration.metadata.depends_on {
            self.topo_visit(dep, executed, visiting, visited, result)?;
        }
        visiting.remove(checksum);
        visited.insert(checksum.to_string());
        result.push(checksum.to_string());
        Ok(())
    }

    /// Builds a map of available migrations by reading the embedded migrations directory and parsing the metadata and SQL files for each migration. Returns an error if two migrations share the same checksum.
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
            if migrations.contains_key(&metadata.checksum) {
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Duplicate migration checksum detected: {}",
                        metadata.checksum
                    ),
                )));
            }
            migrations.insert(
                metadata.checksum.clone(),
                DbMigration::new(metadata, up_sql.to_string(), down_sql.to_string()),
            );
        }
        Ok(migrations)
    }
}
