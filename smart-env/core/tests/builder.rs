#[cfg(test)]
mod tests {
    use smart_env_core::ConfigBuilder;
    use std::env;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_builder_priority_system_over_file() {
        let temp_env_path = ".env.test_priority";
        let mut file = File::create(temp_env_path).expect("Nie można utworzyć pliku testowego");
        writeln!(file, "APP_PORT=3000").unwrap();
        writeln!(file, "DATABASE_URL=postgres://local").unwrap();

        env::set_var("DATABASE_URL", "postgres://production");

        let store = ConfigBuilder::new()
            .load_env_file(temp_env_path)
            .load_system_env()
            .build();

        assert_eq!(store.get("APP_PORT").unwrap(), "3000");
        assert_eq!(store.get("DATABASE_URL").unwrap(), "postgres://production");

        std::fs::remove_file(temp_env_path).unwrap();
        env::remove_var("DATABASE_URL");
    }

    #[test]
    fn test_empty_builder() {
        let store = ConfigBuilder::new().build();
        assert!(store.get("").is_none());
    }
}
