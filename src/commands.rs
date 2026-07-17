use crate::stopwatch::Stopwatch;
use std::time::Duration;

pub enum Command {
    Start,
    Pause,
    Stop,
    Status,
    Reset,
}

pub fn execute(command: Command, stopwatch: &mut Stopwatch) {
    match command {
        Command::Start => {
            if stopwatch.is_running() {
                println!("The stopwatch is already running.");
            } else {
                stopwatch.start();
                println!("▶ Stopwatch started.");
            }
        }

        Command::Pause => {
            if stopwatch.is_running() {
                stopwatch.pause();
                println!(
                    "⏸ Stopwatch paused. Elapsed: {}",
                    format_duration(stopwatch.elapsed())
                );
            } else {
                println!("The stopwatch is already paused.");
            }
        }

        Command::Stop => {
            if stopwatch.is_running() {
                stopwatch.stop();
                println!(
                    "■ Stopwatch stopped. Total time: {}",
                    format_duration(stopwatch.elapsed())
                );
            } else {
                println!("No stopwatch is running.");
            }
        }

        Command::Status => {
            let status = if stopwatch.is_running() {
                "Running"
            } else {
                "Paused"
            };

            println!("Status : {}", status);
            println!("Elapsed: {}", format_duration(stopwatch.elapsed()));
        }

        Command::Reset => {
            stopwatch.reset();
            println!("↺ Stopwatch reset.");
        }
    }
}

fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();

    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    let milliseconds = duration.subsec_millis();

    if hours > 0 {
        format!(
            "{:02}:{:02}:{:02}.{:03}",
            hours, minutes, seconds, milliseconds
        )
    } else {
        format!("{:02}:{:02}.{:03}", minutes, seconds, milliseconds)
    }
}
