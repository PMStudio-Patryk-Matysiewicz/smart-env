use smart_env_macros::Config;

#[derive(Config, Debug, PartialEq)]
struct DbConfig {
    host: String,
    port: u16,
}

#[derive(Config, Debug, PartialEq)]
struct ServerConfig {
    host: String,

    #[config(default = "8080")]
    port: u16,
}

#[derive(Config, Debug, PartialEq)]
struct AppConfig {
    #[config(nested, prefix = "DB_")]
    db: DbConfig,

    #[config(nested, prefix = "SERVER_")]
    server: ServerConfig,
}

#[test]
fn test_nested_config_success() {
    let config = AppConfig::try_load_from_vars([
        ("DB_HOST", "127.0.0.1"),
        ("DB_PORT", "5432"),
        ("SERVER_HOST", "0.0.0.0"),
    ])
    .unwrap();

    assert_eq!(config.db.host, "127.0.0.1");
    assert_eq!(config.db.port, 5432);
    assert_eq!(config.server.host, "0.0.0.0");
    assert_eq!(config.server.port, 8080); 
}

#[test]
fn test_nested_config_missing_field_fails() {
    let result = AppConfig::try_load_from_vars([
        ("DB_HOST", "127.0.0.1"),
        ("SERVER_HOST", "0.0.0.0"),
    ]);

    assert!(result.is_err());
}