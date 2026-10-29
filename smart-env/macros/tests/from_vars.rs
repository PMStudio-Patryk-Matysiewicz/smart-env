use smart_env_macros::Config;

#[derive(Config, Debug, PartialEq)]
struct TestConfig {
    #[config(env = "TEST_HOST")]
    host: String,

    #[config(env = "TEST_PORT", default = "8080")]
    port: u16,
}

#[test]
fn test_try_load_from_vars_success() {
    let result = TestConfig::try_load_from_vars([
        ("TEST_HOST", "10.0.0.1"),
        ("TEST_PORT", "9000"),
    ]);

    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.host, "10.0.0.1");
    assert_eq!(config.port, 9000);
}

#[test]
fn test_try_load_from_vars_default_fallback() {
    let config = TestConfig::try_load_from_vars([
        ("TEST_HOST", "127.0.0.1"),
    ])
    .unwrap();

    assert_eq!(config.host, "127.0.0.1");
    assert_eq!(config.port, 8080);
}

#[test]
fn test_try_load_from_vars_missing_required() {
    let result = TestConfig::try_load_from_vars([
        ("TEST_PORT", "9000"),
    ]);

    assert!(result.is_err());
}

#[test]
fn test_load_from_vars_panics_on_missing() {
    let result = std::panic::catch_unwind(|| {
        TestConfig::load_from_vars(Vec::<(&str, &str)>::new());
    });

    assert!(result.is_err());
}