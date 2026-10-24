use crate::error::EnvError;

pub trait FromEnv: Sized {
    fn parse(s: &str) -> Result<Self, EnvError>;
}

impl FromEnv for String {
    fn parse(s: &str) -> Result<Self, EnvError> {
        Ok(s.to_string())
    }
}

impl FromEnv for bool {
    fn parse(s: &str) -> Result<Self, EnvError> {
        s.parse().map_err(|_| EnvError::ParseError(s.into()))
    }
}

impl FromEnv for u16 {
    fn parse(s: &str) -> Result<Self, EnvError> {
        s.parse().map_err(|_| EnvError::ParseError(s.into()))
    }
}

impl FromEnv for u32 {
    fn parse(s: &str) -> Result<Self, EnvError> {
        s.parse().map_err(|_| EnvError::ParseError(s.into()))
    }
}
