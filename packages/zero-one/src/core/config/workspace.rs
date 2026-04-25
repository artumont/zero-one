use std::io::ErrorKind;

use serde::{Deserialize, Serialize};

use crate::utils::ensure_zero_one_dir;

// A common interface for all configuration objects in the application.
// trait ConfigObject {
//     fn get_docs(&self) -> String;
// }

/// The struct that holds the workspace configuration settings.
#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceConfig {}

impl WorkspaceConfig {
    pub fn create(&self) -> Result<(), Box<dyn std::error::Error>> {
        let zero_one_dir = ensure_zero_one_dir()?;
        let config_path = zero_one_dir.join("config.json");
        if config_path.exists() {
            log::error!(
                "Workspace configuration already exists at {:?}",
                config_path
            );
            return Err(Box::new(std::io::Error::new(
                ErrorKind::AlreadyExists,
                "Workspace configuration already exists.",
            )));
        }
        self.save()?;
        log::debug!("Created new project configuration: {:?}", self);
        Ok(())
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_str = serde_json::to_string(&self)?;
        let zero_one_dir = ensure_zero_one_dir()?;
        let config_path = zero_one_dir.join("config.json");
        std::fs::write(config_path, config_str)?;
        log::debug!("Saved project configuration: {:?}", self);
        Ok(())
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let zero_one_dir = ensure_zero_one_dir()?;
        let config_path = zero_one_dir.join("config.json");
        if !config_path.exists() {
            log::error!(
                "Workspace configuration file not found at {:?}",
                config_path
            );
            return Err("Workspace configuration not found.".into());
        }
        let config_str = std::fs::read_to_string(config_path)?;
        let config: Self = serde_json::from_str(&config_str)?;
        log::debug!("Loaded project configuration: {:?}", config);
        Ok(config)
    }
}
