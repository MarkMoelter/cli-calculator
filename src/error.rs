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
                write!(f, "`{s}` is not a valid operator (use +, -, *, /)")
            }
            CalcError::UnexpectedToken(s) => write!(f, "unexpected `{s}`"),
            CalcError::UnexpectedEnd => write!(f, "unexpected end of input"),
            CalcError::DivideByZero => write!(f, "cannot divide by zero"),
            CalcError::Overflow => write!(f, "result is too large to represent"),
        }
    }
}

impl Error for CalcError {}
