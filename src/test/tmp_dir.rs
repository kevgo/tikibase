use rand::Rng;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

/// creates a temporary directory
pub fn tmp_dir() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let rand: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(3)
        .map(char::from)
        .collect();
    let dir = format!("./tmp/{}-{}", timestamp, rand);
    fs::create_dir_all(&dir).unwrap();
    dir
}
