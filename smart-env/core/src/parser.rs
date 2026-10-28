use crate::error::EnvError;

pub trait FromEnv: Sized {
    fn from_env(value: &str) -> Result<Self, EnvError>;
}

impl FromEnv for String {
    fn from_env(value: &str) -> Result<Self, EnvError> {
        Ok(value.to_string())
    }
}

macro_rules! impl_from_env_via_from_str {
    ($($t:ty),*) => {
        $(
            impl FromEnv for $t {
                fn from_env(value: &str) -> Result<Self, EnvError> {
                    value.parse().map_err(|_| EnvError::ParseError(value.to_string()))
                }
            }
        )*
    };
}

impl_from_env_via_from_str!(
    bool,
    char,
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_all_primitive_types() {
        assert_eq!(String::from_env("hello").unwrap(), "hello");
        assert_eq!(char::from_env("A").unwrap(), 'A');
        assert_eq!(bool::from_env("true").unwrap(), true);
        assert_eq!(bool::from_env("false").unwrap(), false);
        assert_eq!(u8::from_env("255").unwrap(), 255);
        assert_eq!(u16::from_env("65535").unwrap(), 65535);
        assert_eq!(u32::from_env("4294967295").unwrap(), 4294967295);
        assert_eq!(u64::from_env("18446744073709551615").unwrap(), 18446744073709551615);
        assert_eq!(usize::from_env("100").unwrap(), 100);
        assert_eq!(i8::from_env("-128").unwrap(), -128);
        assert_eq!(i16::from_env("-32768").unwrap(), -32768);
        assert_eq!(i32::from_env("-2147483648").unwrap(), -2147483648);
        assert_eq!(i64::from_env("-9223372036854775808").unwrap(), -9223372036854775808);
        assert_eq!(isize::from_env("-50").unwrap(), -50);
        assert_eq!(f32::from_env("3.14").unwrap(), 3.14_f32);
        assert_eq!(f64::from_env("2.7182818284").unwrap(), 2.7182818284_f64);
    }
}
