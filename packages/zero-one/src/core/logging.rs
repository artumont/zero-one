// src/core/logging.rs
use log::LevelFilter;
use std::fs;

#[cfg(not(debug_assertions))]
static LEVEL_FILTER: LevelFilter = LevelFilter::Info;
#[cfg(debug_assertions)]
static LEVEL_FILTER: LevelFilter = LevelFilter::Debug;

/// Initializes the logging system using the `fern` crate. Logs are written to both the console and a log file located in the application's data directory.
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let log_dir = crate::utils::ensure_data_directory()?.join("logs");
    fs::create_dir_all(&log_dir)?;

    let log_file = log_dir.join(format!(
        "zero-one-{}.log",
        chrono::Local::now().format("%Y%m%d")
    ));

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(LEVEL_FILTER)
        .chain(std::io::stdout())
        .chain(fern::log_file(&log_file)?)
        .apply()?;

    Ok(())
}
