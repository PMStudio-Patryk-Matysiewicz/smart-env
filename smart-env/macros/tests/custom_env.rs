use smart_env_macros::Config;

#[derive(Config, Debug, PartialEq)]
struct CustomEnvConfig {
    #[config(env = "DATABASE_URL")]
    db_url: String,
    #[config(env = "APP_PORT", default = "8080")]
    port: u16,
    host: String,
}

#[test]
fn test_custom_env_attribute() {
    unsafe {
        std::env::set_var("DATABASE_URL", "postgres://localhost:5432/mydb");
        std::env::set_var("HOST", "127.0.0.1");
        std::env::remove_var("APP_PORT");
    }

    let config = CustomEnvConfig::load();

    assert_eq!(config.db_url, "postgres://localhost:5432/mydb");
    assert_eq!(config.host, "127.0.0.1");
    assert_eq!(config.port, 8080);

    unsafe {
        std::env::remove_var("DATABASE_URL");
        std::env::remove_var("HOST");
    }
}