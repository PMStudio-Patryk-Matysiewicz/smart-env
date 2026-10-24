use crate::{error::EnvError, parser::FromEnv};
use std::collections::HashMap;

pub struct EnvStore {
    pub(crate) variables: HashMap<String, String>,
}

impl EnvStore {
    pub fn get_parsed<T: FromEnv>(&self, key: &str) -> Result<T, EnvError> {
        let raw_val = self
            .variables
            .get(key)
            .ok_or_else(|| EnvError::MissingVar(key.to_string()))?;

        T::parse(raw_val)
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }
}

impl From<HashMap<String, String>> for EnvStore {
    fn from(variables: HashMap<String, String>) -> Self {
        Self { variables }
    }
}
