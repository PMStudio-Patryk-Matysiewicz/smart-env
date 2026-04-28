use std::collections::HashMap;
use std::sync::OnceLock;

static ENV_FILE_VALUES: OnceLock<HashMap<String, String>> = OnceLock::new();

pub fn load_env_file() {
    let map = crate::env_file::read_env_file(".env");
    ENV_FILE_VALUES.set(map).ok();
}

pub fn get_from_env_file(key: &str) -> Option<String> {
    ENV_FILE_VALUES
        .get()
        .and_then(|m| m.get(key).cloned())
}
