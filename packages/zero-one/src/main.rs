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
    let cli = Cli::parse();
    match cli.command {
        Some(command) => command.resolve(),
        None => cli::commands::session::StartSession {}.run(),
    }
}
