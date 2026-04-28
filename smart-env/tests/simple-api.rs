use smart_env::{init, get, get_or, explain, Source};
use std::fs;

use temp_env::with_var;

#[test]
fn test_get_from_system_env() {
    with_var("TEST_PORT", Some("1234"), || {

        let val: u16 = get("TEST_PORT").unwrap();
        assert_eq!(val, 1234);

        let info = explain("TEST_PORT");
        assert_eq!(info.source, Source::System);
    });
}

// getting from .env test
#[test]
fn test_get_from_env_file() {
    fs::write(".env", "API_KEY=xyz").unwrap();

    init();

    let val: String = get("API_KEY").unwrap();
    assert_eq!(val, "xyz");

    let info = explain("API_KEY");
    assert_eq!(info.source, Source::EnvFile);

    fs::remove_file(".env").unwrap();
}

//missing var test
#[test]
fn test_get_or_default() {
    let val: u16 = get_or("MISSING_VAR", 999);
    assert_eq!(val, 999);

    let info = explain("MISSING_VAR");
    assert_eq!(info.source, Source::Missing);
}