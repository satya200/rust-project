use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Reads the property value for a given key from a simple key=value file.
pub fn get_device_prop<P: AsRef<Path>>(path: P, key: &str) -> Option<String> {
    let file = File::open(path).ok()?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.ok()?.trim().to_string();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == key {
                return Some(v.trim().trim_matches('"').to_string());
            }
        }
    }

    None
}

