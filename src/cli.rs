use crate::{
    commands::{Command, execute},
    storage,
};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "stopwatch", version, about = "A simple stopwatch CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a stopwatch
    Start { name: Option<String> },
    /// Pause a stopwatch
    Pause { name: Option<String> },
    /// Stop a stopwatch
    Stop { name: Option<String> },
    /// Check status of a stopwatch or list all
    Status { name: Option<String> },
    /// Reset a stopwatch
    Reset { name: Option<String> },
    /// Delete a stopwatch
    Delete { name: Option<String> },
    /// List all stopwatches
    List,
}

pub fn run() {
    let cli = Cli::parse();

    let mut stopwatches = storage::load_all().unwrap_or_default();

    let command = match cli.command {
        Commands::Start { name } => Command::Start {
            name: name.unwrap_or_else(|| "default".to_string()),
        },
        Commands::Pause { name } => Command::Pause {
            name: name.unwrap_or_else(|| "default".to_string()),
        },
        Commands::Stop { name } => Command::Stop {
            name: name.unwrap_or_else(|| "default".to_string()),
        },
        Commands::Status { name } => Command::Status { name },
        Commands::Reset { name } => Command::Reset {
            name: name.unwrap_or_else(|| "default".to_string()),
        },
        Commands::Delete { name } => Command::Delete {
            name: name.unwrap_or_else(|| "default".to_string()),
        },
        Commands::List => Command::List,
    };

    execute(command, &mut stopwatches);

    if let Err(err) = storage::save_all(&stopwatches) {
        eprintln!("Failed to save stopwatches: {err}");
    }
}
