use crate::state::get_from_env_file;

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
