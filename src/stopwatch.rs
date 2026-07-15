use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::storage::now_ms;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Stopwatch {
    elapsed_ms: u64,
    running: bool,
    started_at_ms: Option<u128>,
}

impl Stopwatch {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self) {
        if self.running {
            return;
        }

        self.running = true;
        self.started_at_ms = Some(now_ms());
    }

    pub fn pause(&mut self) {
        if !self.running {
            return;
        }

        if let Some(started) = self.started_at_ms {
            let elapsed = now_ms() - started;
            self.elapsed_ms += elapsed as u64;
        }

        self.running = false;
        self.started_at_ms = None;
    }

    pub fn stop(&mut self) {
        self.pause();
    }

    pub fn reset(&mut self) {
        self.elapsed_ms = 0;
        self.running = false;
        self.started_at_ms = None;
    }

    pub fn elapsed(&self) -> Duration {
        let mut total = self.elapsed_ms;

        if self.running {
            if let Some(started) = self.started_at_ms {
                total += (now_ms() - started) as u64;
            }
        }

        Duration::from_millis(total)
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}
