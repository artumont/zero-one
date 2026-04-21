use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// A common interface for all configuration objects in the application.
// trait ConfigObject {
//     fn get_docs(&self) -> String;
// }

/// The struct that holds the project configuration settings.
#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectConfig {}

impl ProjectConfig {
    pub fn create(&self) -> Result<(), Box<dyn std::error::Error>> {
        let zero_one_dir = ensure_zero_one_dir()?;
        let config_path = zero_one_dir.join("config.json");
        if config_path.exists() {
            return Err("Project configuration already exists.".into());
        }
        self.save()?;
        Ok(())
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_str = serde_json::to_string(&self)?;
        let zero_one_dir = ensure_zero_one_dir()?;
        let config_path = zero_one_dir.join("config.json");
        std::fs::write(config_path, config_str)?;
        Ok(())
    }

    pub fn load(&self) -> Result<Self, Box<dyn std::error::Error>> {
        let zero_one_dir = ensure_zero_one_dir()?;
        let config_path = zero_one_dir.join("config.json");
        if !config_path.exists() {
            return Err("Project configuration not found.".into());
        }
        let config_str = std::fs::read_to_string(config_path)?;
        let config: Self = serde_json::from_str(&config_str)?;
        Ok(config)
    }
}

fn ensure_zero_one_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir()?;
    let zero_one_dir = current_dir.join(".zero-one");
    if !zero_one_dir.exists() {
        std::fs::create_dir(&zero_one_dir)?;
    }
    Ok(zero_one_dir)
}
