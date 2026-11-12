use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AocError {
    /// Failure to parse numeric field
    ParseInt(ParseIntError),
    /// The input is missing or semantically incorrect
    Invalid(String),
    /// No solution was found with the given input
    NoSolution,
    /// No solver exists for the requested year/day
    NotImplemented(&'static str),
}

impl fmt::Display for AocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AocError::ParseInt(e) => write!(f, "invalid number: {e}"),
            AocError::Invalid(what) => write!(f, "invalid input: {what}"),
            AocError::NoSolution => write!(f, "no solution found"),
            AocError::NotImplemented(what) => write!(f, "not implemented: {what}"),
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
    /// Map `None` to [`AocError::Invalid`] with a description of what was
    /// expected. The description may be built dynamically.
    fn ctx(self, what: impl Into<String>) -> Result<T, AocError>;
}

impl<T> OptionExt<T> for Option<T> {
    fn ctx(self, what: impl Into<String>) -> Result<T, AocError> {
        self.ok_or_else(|| AocError::Invalid(what.into()))
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
    fn option_ctx_none_becomes_invalid() {
        let v: Option<u32> = None;
        assert_eq!(v.ctx("value"), Err(AocError::Invalid("value".to_string())));
    }

    #[test]
    fn option_ctx_accepts_owned_string() {
        let v: Option<u32> = None;
        assert_eq!(
            v.ctx(format!("value {}", 1)),
            Err(AocError::Invalid("value 1".to_string()))
        );
    }

    #[test]
    fn parse_int_error_converts_via_from() {
        let err: AocError = "x".parse::<i32>().unwrap_err().into();
        assert!(matches!(err, AocError::ParseInt(_)));
    }

    #[test]
    fn displays_are_readable() {
        assert_eq!(
            AocError::Invalid("color \"x\"".to_string()).to_string(),
            "invalid input: color \"x\""
        );
        assert_eq!(AocError::NoSolution.to_string(), "no solution found");
        assert_eq!(
            AocError::NotImplemented("day").to_string(),
            "not implemented: day"
        );
    }
}
