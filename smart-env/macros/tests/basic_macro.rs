use std::env;
use smart_env_macros::Config;

#[derive(Config, Debug, PartialEq)]
struct AppConfig {
    server_host: String,
    server_port: u16,
    enable_metrics: bool,
}

#[test]
fn test_derive_config_loads_from_env() {
    unsafe {
        env::set_var("SERVER_HOST", "127.0.0.1");
        env::set_var("SERVER_PORT", "8080");
        env::set_var("ENABLE_METRICS", "true");
    }

    let config = AppConfig::load();
    
    assert_eq!(config.server_host, "127.0.0.1");
    assert_eq!(config.server_port, 8080);
    assert!(config.enable_metrics);

    unsafe {
        env::remove_var("SERVER_HOST");
        env::remove_var("SERVER_PORT");
        env::remove_var("ENABLE_METRICS");
    }
}