use smart_env_macros::Config;
use temp_env::with_vars;

#[derive(Config, Debug, PartialEq)]
struct TryConfig {
    #[config(env = "TRY_TEST_HOST")]
    host: String,

    #[config(env = "TRY_TEST_PORT_MISSING")]
    port: u16,
}

#[test]
fn test_try_load_success() {
    with_vars(
        [
            ("TRY_TEST_HOST", Some("127.0.0.1")),
            ("TRY_TEST_PORT_MISSING", Some("8080")),
        ],
        || {
            let result = TryConfig::try_load();
            assert!(result.is_ok());

            let config = result.unwrap();
            assert_eq!(config.host, "127.0.0.1");
            assert_eq!(config.port, 8080);
        },
    );
}

#[test]
fn test_try_load_missing_var_returns_err() {
    with_vars(
        [
            ("TRY_TEST_HOST", Some("127.0.0.1")),
            ("TRY_TEST_PORT_MISSING", None::<&str>),
        ],
        || {
            let result = TryConfig::try_load();
            assert!(result.is_err());
        },
    );
}