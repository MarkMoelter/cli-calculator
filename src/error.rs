use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalcError {
    InvalidCharacter(char),
    InvalidNumber(String),
    InvalidToken(String),
    UnexpectedToken(String),
    UnexpectedEnd,
    DivideByZero,
    Overflow,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcError::InvalidCharacter(c) => write!(f, "`{c}` is not a valid character"),
            CalcError::InvalidNumber(s) => write!(f, "`{s}` is not a valid number"),
            CalcError::InvalidToken(s) => {
                write!(f, "`{s}` is not a valid token")
            }
            CalcError::UnexpectedToken(s) => write!(f, "unexpected `{s}`"),
            CalcError::UnexpectedEnd => write!(f, "unexpected end of input"),
            CalcError::DivideByZero => write!(f, "cannot divide by zero"),
            CalcError::Overflow => write!(f, "result is too large to represent"),
        }
    }
}

impl Error for CalcError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_messages() {
        assert_eq!(
            CalcError::UnexpectedToken("*".into()).to_string(),
            "unexpected `*`"
        );
        assert_eq!(
            CalcError::UnexpectedEnd.to_string(),
            "unexpected end of input"
        );
        assert_eq!(CalcError::DivideByZero.to_string(), "cannot divide by zero");
        assert_eq!(
            CalcError::InvalidCharacter('~').to_string(),
            "`~` is not a valid character"
        );
        assert_eq!(
            CalcError::InvalidNumber("a".to_string()).to_string(),
            "`a` is not a valid number"
        );
        assert_eq!(
            CalcError::InvalidToken("~".to_string()).to_string(),
            "`~` is not a valid token"
        );
        assert_eq!(
            CalcError::Overflow.to_string(),
            "result is too large to represent"
        )
    }
}
