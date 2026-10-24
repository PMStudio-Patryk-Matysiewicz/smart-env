use std::collections::HashMap;
use std::fs;

pub fn read_env_file(path: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();

    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return map, // brak pliku = pusta mapa
    };

    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            map.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    map
}
