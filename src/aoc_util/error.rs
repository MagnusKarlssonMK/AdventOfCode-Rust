use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AocError {
    /// Mandatory part of the input missing
    Missing(&'static str),
    /// Failure to parse numeric field
    ParseInt(ParseIntError),
    /// The input is semantically incorrect
    Invalid(String),
}

impl fmt::Display for AocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AocError::Missing(what) => write!(f, "missing input: {what}"),
            AocError::ParseInt(e) => write!(f, "invalid number: {e}"),
            AocError::Invalid(what) => write!(f, "invalid input: {what}"),
        }
    }
}

impl Error for AocError {}

impl From<ParseIntError> for AocError {
    fn from(value: ParseIntError) -> Self {
        AocError::ParseInt(value)
    }
}

/// Extension trait to turn an `Option` into a `Result<_, AocError>` for
/// use with `?`, keeping conversions as terse as the old `unwrap()`.
pub trait OptionExt<T> {
    /// Map 'None' to [`AocError::Missing`] with a static description of
    /// what was expected.
    fn ctx(self, what: &'static str) -> Result<T, AocError>;
}

impl<T> OptionExt<T> for Option<T> {
    fn ctx(self, what: &'static str) -> Result<T, AocError> {
        self.ok_or(AocError::Missing(what))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_ctx_some_passes_through() {
        let v: Option<u32> = Some(42);
        assert_eq!(v.ctx("value"), Ok(42));
    }

    #[test]
    fn option_ctx_none_becomes_missing() {
        let v: Option<u32> = None;
        assert_eq!(v.ctx("value"), Err(AocError::Missing("value")));
    }

    #[test]
    fn parse_int_error_converts_via_from() {
        let err: AocError = "x".parse::<i32>().unwrap_err().into();
        assert!(matches!(err, AocError::ParseInt(_)));
    }

    #[test]
    fn displays_are_readable() {
        assert_eq!(
            AocError::Missing("price x").to_string(),
            "missing input: price x"
        );
        assert_eq!(
            AocError::Invalid("color \"x\"".to_string()).to_string(),
            "invalid input: color \"x\""
        );
    }
}
