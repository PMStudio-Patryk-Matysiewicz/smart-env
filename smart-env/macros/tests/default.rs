use smart_env_macros::Config;

#[derive(Config, Debug, PartialEq)]
struct DefaultConfig {
    #[config(default = "8080")]
    port: u16,

    #[config(default = "postgres")]
    db_user: String,

    host: String,
}

#[test]
fn test_default_attribute_behavior() {
    unsafe {
        std::env::set_var("HOST", "127.0.0.1");
        std::env::set_var("DB_USER", "admin");
        std::env::remove_var("PORT");
    }

    let config = DefaultConfig::load();

    assert_eq!(config.host, "127.0.0.1");
    assert_eq!(config.db_user, "admin"); 
    assert_eq!(config.port, 8080);

    unsafe {
        std::env::remove_var("HOST");
        std::env::remove_var("DB_USER");
    }
}