use smart_env::Config;

#[derive(Config, Debug, PartialEq)]
struct OptionConfig {
    app_name: String,
    db_timeout: Option<u32>,
    debug_mode: Option<bool>,
}

#[test]
fn test_option_fields_behavior() {
    unsafe {
        std::env::set_var("APP_NAME", "SmartApp");
        std::env::set_var("DB_TIMEOUT", "30");
        std::env::remove_var("DEBUG_MODE");
    }

    let config = OptionConfig::load();

    assert_eq!(config.app_name, "SmartApp");
    assert_eq!(config.db_timeout, Some(30));
    assert_eq!(config.debug_mode, None); 

    unsafe {
        std::env::remove_var("APP_NAME");
        std::env::remove_var("DB_TIMEOUT");
    }
}