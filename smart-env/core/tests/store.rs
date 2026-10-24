#[cfg(test)]
mod tests {
    use smart_env_core::{EnvError, EnvStore};
    use std::collections::HashMap;

    #[test]
    fn test_get_parsed_success() {
        let mut vars = HashMap::new();
        vars.insert("PORT".to_string(), "8080".to_string());
        vars.insert("DEBUG".to_string(), "true".to_string());

        let store: EnvStore = vars.into();

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
        let store: EnvStore = HashMap::new().into();

        let result: Result<String, EnvError> = store.get_parsed("API_KEY");

        assert!(matches!(result, Err(EnvError::MissingVar(_))));
    }

    #[test]
    fn test_get_parsed_invalid_type() {
        let mut vars = HashMap::new();
        vars.insert("PORT".to_string(), "abc".to_string());

        let store: EnvStore = vars.into();

        let result: Result<u16, EnvError> = store.get_parsed("PORT");

        // Weryfikacja czy literówki rzucają błąd parsowania
        assert!(matches!(result, Err(EnvError::ParseError(_))));
    }
}
