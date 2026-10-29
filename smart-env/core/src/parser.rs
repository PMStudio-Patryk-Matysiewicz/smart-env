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

impl<T: FromEnv> FromEnv for Vec<T> {
    fn from_env(s: &str) -> Result<Self, EnvError> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Ok(Vec::new());
        }

        trimmed
            .split(',')
            .map(|part| T::from_env(part.trim()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_all_primitive_types() {
        assert_eq!(String::from_env("hello").unwrap(), "hello");
        assert_eq!(char::from_env("A").unwrap(), 'A');
        assert!(bool::from_env("true").unwrap());
        assert!(!bool::from_env("false").unwrap());
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
        assert_eq!(f32::from_env("12.34").unwrap(), 12.34_f32);
        assert_eq!(f64::from_env("56.789").unwrap(), 56.789_f64);
    }

    #[test]
    fn test_parse_vec_of_strings() {
        let raw = "127.0.0.1, 192.168.1.1, 10.0.0.1";
        let parsed: Vec<String> = FromEnv::from_env(raw).unwrap();

        assert_eq!(parsed, vec!["127.0.0.1", "192.168.1.1", "10.0.0.1"]);
    }

    #[test]
    fn test_parse_vec_of_numbers() {
        let raw = "80, 443, 8080";
        let parsed: Vec<u16> = FromEnv::from_env(raw).unwrap();

        assert_eq!(parsed, vec![80, 443, 8080]);
    }

    #[test]
    fn test_parse_empty_vec() {
        let raw = "   ";
        let parsed: Vec<i32> = FromEnv::from_env(raw).unwrap();

        assert!(parsed.is_empty());
    }

    #[test]
    fn test_parse_invalid_element_fails() {
        let raw = "80, invalid_port, 8080";
        let result = Vec::<u16>::from_env(raw);

        assert!(result.is_err());
    }
}
