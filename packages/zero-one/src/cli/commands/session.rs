use std::process::ExitCode;

use clap::Args;

use crate::cli::tui;

/// This will start a new session and open the TUI interface
#[derive(Args)]
pub struct StartSession {}

impl StartSession {
    pub fn run(self) -> ExitCode {
        let current_directory = std::env::current_dir();
        if current_directory.is_err() {
            eprintln!("Failed to get current directory");
            return ExitCode::FAILURE;
        }
        // TODO: Add session init here
        if tui::run_tui().is_err() {
            eprintln!("Failed to start TUI");
            return ExitCode::FAILURE;
        }
        ExitCode::SUCCESS
    }
}
