pub mod commands;
pub mod tui;
use clap::Subcommand;
use std::process::ExitCode;

/// This macro is used to define the available CLI commands for the application. Each command is represented
/// as a variant of the `Commands` enum, and the associated type contains the arguments for that command.
///
/// To add a new command, define a struct for the command's arguments and implement a `run` method for it.
/// Then, add the command to the `command_registry!` macro at the bottom of this file.
///
/// ## Example:
/// ```
/// #[derive(Args)]
/// pub struct Greet {
///     #[arg(short, long)]
///     pub name: String,
/// }
///
/// impl Greet {
///     pub fn run(self) {
///         println!("Hello, {}!", self.name);
///     }
/// }
/// ```
///
/// ## Definition:
/// ```
/// command_registry! {
///    Greet(Greet),
/// }
/// ```
macro_rules! command_registry {
    (
        $( $variant:ident ( $ty:ty ) ),* $(,)?
    ) => {
        #[derive(Subcommand)]
        pub enum Commands {
            $( $variant($ty), )*
        }

        impl Commands {
            pub fn resolve(self) -> ExitCode {
                match self {
                    $( Commands::$variant(cmd) => cmd.run(), )*
                }
            }
        }
    };
}

command_registry! {
    StartSession(commands::session::StartSession),
}
