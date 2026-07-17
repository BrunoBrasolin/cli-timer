use crate::stopwatch::Stopwatch;
use std::{
    fs, io,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

const FILE_NAME: &str = ".stopwatch.json";

pub fn load() -> io::Result<Stopwatch> {
    let path = storage_path();

    if !path.exists() {
        return Ok(Stopwatch::new());
    }

    let contents = fs::read_to_string(path)?;
    let stopwatch = serde_json::from_str(&contents).map_err(io::Error::other)?;

    Ok(stopwatch)
}

pub fn save(stopwatch: &Stopwatch) -> io::Result<()> {
    let json = serde_json::to_string_pretty(stopwatch).map_err(io::Error::other)?;
    fs::write(storage_path(), json)
}

pub fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

fn storage_path() -> PathBuf {
    PathBuf::from(FILE_NAME)
}
