use crate::{
    commands::{Command, execute},
    stopwatch::Stopwatch,
    storage,
};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "stopwatch", version, about = "A simple sotpwatch CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Start,
    Pause,
    Stop,
    Status,
    Reset,
}

pub fn run() {
    let cli = Cli::parse();

    let mut stopwatch = storage::load().unwrap_or_else(|_| Stopwatch::new());

    let command = match cli.command {
        Commands::Start => Command::Start,
        Commands::Pause => Command::Pause,
        Commands::Stop => Command::Stop,
        Commands::Status => Command::Status,
        Commands::Reset => Command::Reset,
    };

    execute(command, &mut stopwatch);

    if let Err(err) = storage::save(&stopwatch) {
        eprintln!("Failed to save stopwatch: {err}");
    }
}
