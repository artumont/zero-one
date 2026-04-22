use std::process::ExitCode;

use clap::Args;

use crate::cli::tui;

/// This will start a new session and open the TUI interface
#[derive(Args, Debug)]
pub struct StartSession {}

impl StartSession {
    pub fn run(self) -> ExitCode {
        let current_directory = std::env::current_dir();
        if current_directory.is_err() {
            log::error!("Failed to get current directory");
            return ExitCode::FAILURE;
        }
        // TODO: Add session init here and pass it onto the TUI
        if let Err(err) = tui::run_tui() {
            log::error!("Failed to start TUI: {err:?}");
            return ExitCode::FAILURE;
        }
        ExitCode::SUCCESS
    }
}
