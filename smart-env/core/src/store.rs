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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_parsed_success() {
        let mut vars = HashMap::new();
        vars.insert("PORT".to_string(), "8080".to_string());
        vars.insert("DEBUG".to_string(), "true".to_string());

        let store = EnvStore { variables: vars };

        let port: u16 = store
            .get_parsed("PORT")
            .expect("Parsowanie PORT nie powiodło się");
        assert_eq!(port, 8080);

        let debug: bool = store
            .get_parsed("DEBUG")
            .expect("Parsowanie DEBUG nie powiodło się");
        assert!(debug);
    }

    #[test]
    fn test_get_parsed_missing_variable() {
        let store = EnvStore { variables: HashMap::new() };
        let result: Result<String, EnvError> = store.get_parsed("API_KEY");

        assert!(matches!(result, Err(EnvError::MissingVar(_))));
    }

    #[test]
    fn test_get_parsed_invalid_type() {
        let mut vars = HashMap::new();
        vars.insert("PORT".to_string(), "abc".to_string());

        let store = EnvStore { variables: vars };
        let result: Result<u16, EnvError> = store.get_parsed("PORT");

        assert!(matches!(result, Err(EnvError::ParseError(_))));
    }
}
