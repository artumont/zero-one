const APP_NAME: &str = "zero-one";

/// Ensures that the application data directory exists and returns its path. If the directory does not exist, it will be created.
pub fn ensure_data_directory() -> String {
    #[cfg(target_os = "windows")]
    {
        let appdata = std::env::var("APPDATA").expect("APPDATA environment variable not set");
        let data_dir = format!("{}\\{}", appdata, APP_NAME);
        std::fs::create_dir_all(&data_dir).expect("Failed to create data directory");
        return data_dir;
    }
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").expect("HOME environment variable not set");
        let data_dir = format!("{}/Library/Application Support/{}", home, APP_NAME);
        std::fs::create_dir_all(&data_dir).expect("Failed to create data directory");
        return data_dir;
    }
    #[cfg(target_os = "linux")]
    {
        let home = std::env::var("HOME").expect("HOME environment variable not set");
        let data_dir = format!("{}/.local/share/{}", home, APP_NAME);
        std::fs::create_dir_all(&data_dir).expect("Failed to create data directory");
        return data_dir;
    }
}

/// Ensures that the application database file exists and returns its path. If the file does not exist, it will be created.
pub fn ensure_app_database() -> String {
    let data_dir = ensure_data_directory();
    let db_path = format!("{}/zero-one.db", data_dir);
    if !std::path::Path::new(&db_path).exists() {
        std::fs::File::create(&db_path).expect("Failed to create database file");
    }
    db_path
}
