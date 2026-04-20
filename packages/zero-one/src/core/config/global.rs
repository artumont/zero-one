use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// A common interface for all configuration objects in the application.
trait ConfigObject {
    fn get_docs(&self) -> String;
}

/// The struct that holds the application's configuration settings.
#[derive(Serialize, Deserialize)]
pub struct GlobalConfig {}

impl GlobalConfig {
    pub fn save(self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let config_str = serde_json::to_string(&self)?;
        let config_path = path.join("zero-one-config.json");
        std::fs::write(config_path, config_str)?;
        Ok(())
    }

    pub fn load(self, path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = path.join("zero-one-config.json");
        let config_str = std::fs::read_to_string(config_path)?;
        let config: Self = serde_json::from_str(&config_str)?;
        Ok(config)
    }
}
