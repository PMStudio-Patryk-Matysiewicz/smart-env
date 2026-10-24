use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum EnvError {
    #[error("Missing required environment variable: {0}")]
    #[diagnostic(
        code(smart_env::missing_var),
        help("Make sure this variable is exported in your system or defined in the .env file.")
    )]
    MissingVar(String),

    #[error("Failed to parse environment variable")]
    #[diagnostic(
        code(smart_env::parse_error),
        help("Check if the value matches the expected type (e.g., u16, bool).")
    )]
    ParseError(String),
}
