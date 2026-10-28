use smart_env::{Config, ConfigBuilder, EnvStore};
use temp_env::with_vars;

#[derive(Config, Debug, PartialEq)]
struct FacadeConfig {
    #[config(env = "FACADE_HOST")]
    host: String,

    #[config(env = "FACADE_PORT", default = "8080")]
    port: u16,
}

#[test]
fn test_facade_exports_macro_and_core_types() {
    with_vars(
        [
            ("FACADE_HOST", Some("127.0.0.1")),
        ],
        || {
            let config = FacadeConfig::load();
            assert_eq!(config.host, "127.0.0.1");
            assert_eq!(config.port, 8080);

            let store: EnvStore = ConfigBuilder::new()
                .with_vars([("CUSTOM_KEY", "custom_val")])
                .build();

            let val = store.get_parsed::<String>("CUSTOM_KEY");
            assert_eq!(val.unwrap(), "custom_val");
        },
    );
}