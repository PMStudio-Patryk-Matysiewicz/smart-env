#[derive(Debug)]
pub enum EnvError {
    MissingVar(String),
    ParseError(String),
}
