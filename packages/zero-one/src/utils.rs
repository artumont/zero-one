const APP_NAME: &str = "zero-one";
const APP_VERSION: &str = "0.1.0";

pub fn get_app_name() -> &'static str {
    APP_NAME
}

pub fn get_app_version() -> &'static str {
    APP_VERSION
}

/// Ensures that the application data directory exists and returns its path. If the directory does not exist, it will be created.
pub fn ensure_data_directory() -> Result<std::path::PathBuf, std::io::Error> {
    #[cfg(target_os = "windows")]
    {
        let appdata = std::env::var("APPDATA").expect("APPDATA environment variable not set");
        let data_dir = std::path::PathBuf::from(appdata).join(APP_NAME);
        std::fs::create_dir_all(&data_dir)?;
        return Ok(data_dir);
    }
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").expect("HOME environment variable not set");
        let data_dir = std::path::PathBuf::from(home)
            .join("Library")
            .join("Application Support")
            .join(APP_NAME);
        std::fs::create_dir_all(&data_dir)?;
        return Ok(data_dir);
    }
    #[cfg(target_os = "linux")]
    {
        let home = std::env::var("HOME").expect("HOME environment variable not set");
        let data_dir = std::path::PathBuf::from(home)
            .join(".local")
            .join("share")
            .join(APP_NAME);
        std::fs::create_dir_all(&data_dir)?;
        return Ok(data_dir);
    }
}

/// Ensures that the application database file exists and returns its path. If the file does not exist, it will be created.  
pub fn ensure_app_database() -> Result<std::path::PathBuf, std::io::Error> {
    let data_dir = ensure_data_directory();
    let db_path = data_dir?.join("zero-one.db");
    if !db_path.exists() {
        std::fs::File::create(&db_path).expect("Failed to create database file");
    }
    Ok(db_path)
}
