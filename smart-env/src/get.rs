use crate::{parser::FromEnv, state::get_from_env_file};

pub fn get<T: FromEnv>(key: &str) -> Option<T> {
    // 1. system env
    if let Ok(val) = std::env::var(key) {
        return T::parse(&val).ok();
    }

    // 2. fallback: .env
    if let Some(val) = get_from_env_file(key) {
        return T::parse(&val).ok();
    }

    None
}

pub fn get_or<T: FromEnv>(key: &str, default: T) -> T {
    get(key).unwrap_or(default)
}
