use std::process::Command;

#[test]
fn test_cli_check_command() {
    let bin_path = env!("CARGO_BIN_EXE_smart-env");

    let output = Command::new(bin_path)
        .arg("check")
        .output()
        .expect("Failed to execute CLI binary");

    assert!(output.status.success());
}