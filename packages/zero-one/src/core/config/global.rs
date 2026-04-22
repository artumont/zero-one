use std::io::ErrorKind;

use serde::{Deserialize, Serialize};

// A common interface for all configuration objects in the application.
// trait ConfigObject {
//     fn get_docs(&self) -> String;
// }

/// The struct that holds the application's configuration settings.
#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalConfig {}

impl GlobalConfig {
    pub fn create(&self) -> Result<(), Box<dyn std::error::Error>> {
        let app_data_dir = crate::utils::ensure_data_directory();
        let config_path = app_data_dir?.join("global-config.json");
        if config_path.exists() {
            log::error!("Global configuration already exists at {:?}", config_path);
            return Err(Box::new(std::io::Error::new(
                ErrorKind::AlreadyExists,
                "Global configuration already exists.",
            )));
        }
        log::debug!("Creating new global configuration: {:?}", self);
        self.save()?;
        Ok(())
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_str = serde_json::to_string(&self)?;
        let app_data_dir = crate::utils::ensure_data_directory();
        let config_path = app_data_dir?.join("global-config.json");
        std::fs::write(config_path, config_str)?;
        log::debug!("Saved global configuration: {:?}", self);
        Ok(())
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let app_data_dir = crate::utils::ensure_data_directory();
        let config_path = app_data_dir?.join("global-config.json");
        if !config_path.exists() {
            log::error!("Global configuration file not found at {:?}", config_path);
            return Err("Global configuration not found.".into());
        }
        let config_str = std::fs::read_to_string(config_path)?;
        let config: Self = serde_json::from_str(&config_str)?;
        log::debug!("Loaded global configuration: {:?}", config);
        Ok(config)
    }
}
