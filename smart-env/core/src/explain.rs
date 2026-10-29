use crate::state::get_from_env_file;
use std::collections::BTreeSet;
use crate::ConfigBuilder;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
    System,
    EnvFile,
    Missing,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Explanation {
    pub key: String,
    pub value: Option<String>,
    pub source: Source,
}

pub fn explain(key: &str) -> Explanation {
    if let Ok(val) = std::env::var(key) {
        return Explanation {
            key: key.to_string(),
            value: Some(val),
            source: Source::System,
        };
    }

    if let Some(val) = get_from_env_file(key) {
        return Explanation {
            key: key.to_string(),
            value: Some(val),
            source: Source::EnvFile,
        };
    }

    Explanation {
        key: key.to_string(),
        value: None,
        source: Source::Missing,
    }
}

pub fn explain_all() -> Vec<Explanation> {
    let mut keys = BTreeSet::new();
    keys.extend(std::env::vars().map(|(k, _)| k));

    let env_store = ConfigBuilder::new()
        .load_env_file(".env")
        .build();
    
    keys.extend(env_store.iter().map(|(k, _)| k.clone()));
    keys.into_iter().map(|key| explain(&key)).collect()
}