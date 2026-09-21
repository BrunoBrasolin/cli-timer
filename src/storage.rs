use crate::stopwatch::Stopwatch;
use std::{
    fs, io,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use std::collections::HashMap;

const FILE_NAME: &str = ".stopwatch.json";

pub fn load_all() -> io::Result<HashMap<String, Stopwatch>> {
    let path = storage_path();

    if !path.exists() {
        return Ok(HashMap::new());
    }

    let contents = fs::read_to_string(path)?;
    if let Ok(map) = serde_json::from_str::<HashMap<String, Stopwatch>>(&contents) {
        Ok(map)
    } else if let Ok(single) = serde_json::from_str::<Stopwatch>(&contents) {
        let mut map = HashMap::new();
        map.insert("default".to_string(), single);
        Ok(map)
    } else {
        Ok(HashMap::new())
    }
}

pub fn save_all(stopwatches: &HashMap<String, Stopwatch>) -> io::Result<()> {
    let json = serde_json::to_string_pretty(stopwatches).map_err(io::Error::other)?;
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
