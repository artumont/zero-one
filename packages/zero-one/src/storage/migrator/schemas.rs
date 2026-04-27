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
    up: String, // Contents of the 'up' migration file, e.g., "1777256991_initial_setup/up.sql"
    down: String, // Contents of the 'down' migration file, e.g., "1777256991_initial_setup/down.sql"
}

impl DbMigration {
    pub fn new(metadata: Metadata, up: String, down: String) -> Self {
        Self { metadata, up, down }
    }

    /// Executes the 'up' migration to apply changes to the database.
    pub async fn upgrade(
        &self,
        conn: &rusqlite::Connection,
    ) -> Result<(), Box<dyn std::error::Error>> {
        conn.execute_batch(&self.up)?;
        Ok(())
    }

    /// Executes the 'down' migration to revert the database to the previous state.
    pub async fn downgrade(
        &self,
        conn: &rusqlite::Connection,
    ) -> Result<(), Box<dyn std::error::Error>> {
        conn.execute_batch(&self.down)?;
        Ok(())
    }
}
