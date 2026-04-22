use std::process::ExitCode;

use clap::Parser;
use zero_one::cli::{self};

#[derive(Parser)]
#[command(name = "z1")]
#[command(about = "zero-one CLI version", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<cli::Commands>,
}

fn main() -> ExitCode {
    match zero_one::core::logging::init() {
        Ok(_) => log::info!("Logging initialized successfully"),
        Err(err) => {
            eprintln!("Failed to initialize logging: {err:?}");
            return ExitCode::FAILURE;
        }
    }

    let cli = Cli::parse();
    match cli.command {
        Some(command) => command.resolve(),
        None => cli::commands::session::StartSession {}.run(),
    }
}
