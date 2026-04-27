use std::collections::BTreeSet;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Metadata {
    pub checksum: String, // Checksum of the migration
    pub name: String,
    // pub author: String,
    // pub description: Option<String>,
    pub depends_on: BTreeSet<String>, // Set of checksums that this migration depends on
}

pub struct DbMigration {
    pub metadata: Metadata, // Metadata from the `metadata` file in the migration folder, e.g., "1777256991_initial_setup/metadata.json"
    pub(crate) up: String, // Contents of the 'up' migration file, e.g., "1777256991_initial_setup/up.sql"
    #[allow(dead_code)] // Retained for future rollback support
    pub(crate) down: String, // Contents of the 'down' migration file, e.g., "1777256991_initial_setup/down.sql"
}

/// Strips bare `BEGIN`, `COMMIT`, and `ROLLBACK` statements from SQL so that
/// the migrator can wrap each migration in its own rusqlite transaction.
fn strip_transaction_markers(sql: &str) -> String {
    sql.lines()
        .filter(|line| {
            let t = line.trim().to_uppercase();
            !matches!(
                t.as_str(),
                "BEGIN" | "BEGIN;" | "COMMIT" | "COMMIT;" | "ROLLBACK" | "ROLLBACK;"
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

impl DbMigration {
    pub fn new(metadata: Metadata, up: String, down: String) -> Self {
        Self {
            metadata,
            up: strip_transaction_markers(&up),
            down: strip_transaction_markers(&down),
        }
    }
}
