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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use temp_env::with_vars;

    #[test]
    fn test_builder_priority_system_over_file() {
        let dir = tempfile::tempdir().expect("Nie można utworzyć katalogu tymczasowego");
        let path = dir.path().join(".env");
        fs::write(&path, "APP_PORT=3000\nDATABASE_URL=postgres://local\n").unwrap();

        with_vars(
            [
                ("DATABASE_URL", Some("postgres://production")),
                ("APP_PORT", None::<&str>),
            ],
            || {
                let store = ConfigBuilder::new()
                    .load_env_file(path.to_str().unwrap())
                    .load_system_env()
                    .build();

                assert_eq!(store.get("APP_PORT").unwrap(), "3000");
                assert_eq!(store.get("DATABASE_URL").unwrap(), "postgres://production");
            },
        );
    }

    #[test]
    fn test_empty_builder() {
        let store = ConfigBuilder::new().build();
        assert!(store.get("").is_none());
    }
}
