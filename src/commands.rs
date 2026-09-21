use crate::stopwatch::Stopwatch;
use std::collections::HashMap;
use std::time::Duration;

pub enum Command {
    Start { name: String },
    Pause { name: String },
    Stop { name: String },
    Status { name: Option<String> },
    Reset { name: String },
    Delete { name: String },
    List,
}

pub fn execute(command: Command, stopwatches: &mut HashMap<String, Stopwatch>) {
    match command {
        Command::Start { name } => {
            let stopwatch = stopwatches.entry(name.clone()).or_insert_with(Stopwatch::new);
            if stopwatch.is_running() {
                println!("The stopwatch '{}' is already running.", name);
            } else {
                stopwatch.start();
                println!(
                    "▶ Stopwatch '{}' started. Elapsed: {}",
                    name,
                    format_duration(stopwatch.elapsed())
                );
            }
        }

        Command::Pause { name } => {
            if let Some(stopwatch) = stopwatches.get_mut(&name) {
                if stopwatch.is_running() {
                    stopwatch.pause();
                    println!(
                        "⏸ Stopwatch '{}' paused. Elapsed: {}",
                        name,
                        format_duration(stopwatch.elapsed())
                    );
                } else {
                    println!("The stopwatch '{}' is already paused.", name);
                }
            } else {
                println!("No stopwatch found with name '{}'.", name);
            }
        }

        Command::Stop { name } => {
            if let Some(stopwatch) = stopwatches.get_mut(&name) {
                if stopwatch.is_running() {
                    stopwatch.pause();
                    println!(
                        "■ Stopwatch '{}' stopped. Total time: {}",
                        name,
                        format_duration(stopwatch.elapsed())
                    );
                    stopwatch.reset();
                } else {
                    println!("Stopwatch '{}' is not running.", name);
                }
            } else {
                println!("No stopwatch found with name '{}'.", name);
            }
        }

        Command::Status { name } => {
            if let Some(name) = name {
                if let Some(stopwatch) = stopwatches.get(&name) {
                    let status = if stopwatch.is_running() {
                        "Running"
                    } else {
                        "Paused"
                    };
                    println!("Name   : {}", name);
                    println!("Status : {}", status);
                    println!("Elapsed: {}", format_duration(stopwatch.elapsed()));
                } else {
                    println!("No stopwatch found with name '{}'.", name);
                }
            } else {
                print_list(stopwatches);
            }
        }

        Command::Reset { name } => {
            if let Some(stopwatch) = stopwatches.get_mut(&name) {
                stopwatch.reset();
                println!("↺ Stopwatch '{}' reset.", name);
            } else {
                println!("No stopwatch found with name '{}'.", name);
            }
        }

        Command::Delete { name } => {
            if stopwatches.remove(&name).is_some() {
                println!("🗑 Stopwatch '{}' deleted.", name);
            } else {
                println!("No stopwatch found with name '{}'.", name);
            }
        }

        Command::List => {
            print_list(stopwatches);
        }
    }
}

fn print_list(stopwatches: &HashMap<String, Stopwatch>) {
    if stopwatches.is_empty() {
        println!("No stopwatches found.");
        return;
    }

    let mut names: Vec<&String> = stopwatches.keys().collect();
    names.sort();

    println!("{:<15} {:<10} {:<12}", "NAME", "STATUS", "ELAPSED");
    println!("{}", "-".repeat(37));
    for name in names {
        let sw = &stopwatches[name];
        let status = if sw.is_running() { "Running" } else { "Paused" };
        println!("{:<15} {:<10} {:<12}", name, status, format_duration(sw.elapsed()));
    }
}

fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();

    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    } else {
        format!("00:{:02}:{:02}", minutes, seconds)
    }
}
