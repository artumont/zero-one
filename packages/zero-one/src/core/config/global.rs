use serde::{Deserialize, Serialize};

/// A common interface for all configuration objects in the application.
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
            return Err("Global configuration already exists.".into());
        }
        self.save()?;
        Ok(())
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_str = serde_json::to_string(&self)?;
        let app_data_dir = crate::utils::ensure_data_directory();
        let config_path = app_data_dir?.join("global-config.json");
        std::fs::write(config_path, config_str)?;
        Ok(())
    }

    pub fn load(&self) -> Result<Self, Box<dyn std::error::Error>> {
        let app_data_dir = crate::utils::ensure_data_directory();
        let config_path = app_data_dir?.join("global-config.json");
        if !config_path.exists() {
            return Err("Global configuration not found.".into());
        }
        let config_str = std::fs::read_to_string(config_path)?;
        let config: Self = serde_json::from_str(&config_str)?;
        Ok(config)
    }
}
