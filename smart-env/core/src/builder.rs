use crate::env_file::read_env_file;
use crate::store::EnvStore;
use std::collections::HashMap;

#[derive(Default)]
pub struct ConfigBuilder {
    variables: HashMap<String, String>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn load_env_file(mut self, path: &str) -> Self {
        let file_vars = read_env_file(path);

        for (key, value) in file_vars {
            self.variables.insert(key, value);
        }

        self
    }

    pub fn load_system_env(mut self) -> Self {
        for (key, value) in std::env::vars() {
            self.variables.insert(key, value);
        }

        self
    }

    pub fn build(self) -> EnvStore {
        EnvStore {
            variables: self.variables,
        }
    }
}
