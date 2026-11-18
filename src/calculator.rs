use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalcError {
    InvalidFormat, // expects `<number> <operator> <number>`
    InvalidNumber(String),
    InvalidOperator(String),
    DivideByZero,
    Overflow,
    InvalidCharacter(char),
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcError::InvalidFormat => {
                write!(f, "expected input like `3 + 4` (number operator number)")
            }
            CalcError::InvalidNumber(s) => write!(f, "`{s}` is not a valid number"),
            CalcError::InvalidOperator(s) => {
                write!(f, "`{s}` is not a valid operator (use +, -, *, /)")
            }
            CalcError::DivideByZero => write!(f, "cannot divide by zero"),
            CalcError::Overflow => write!(f, "result is too large to represent"),
            CalcError::InvalidCharacter(c) => write!(f, "`{c}` is not a valid character"),
        }
    }
}

impl Error for CalcError {}

impl FromStr for Operator {
    type Err = CalcError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Subtract),
            "*" => Ok(Operator::Multiply),
            "/" => Ok(Operator::Divide),
            _ => Err(CalcError::InvalidOperator(String::from(s))),
        }
    }
}

pub fn parse_number(input: &str) -> Result<f64, CalcError> {
    input
        .trim()
        .parse::<f64>()
        .ok()
        .filter(|n| n.is_finite())
        .ok_or_else(|| CalcError::InvalidNumber(String::from(input.trim())))
}

pub fn calculate(num_1: f64, num_2: f64, operation: Operator) -> Result<f64, CalcError> {
    let res = match operation {
        Operator::Add => num_1 + num_2,
        Operator::Subtract => num_1 - num_2,
        Operator::Multiply => num_1 * num_2,
        Operator::Divide => {
            if num_2 == 0.0 {
                return Err(CalcError::DivideByZero);
            }
            num_1 / num_2
        }
    };

    if res.is_finite() {
        Ok(res)
    } else {
        Err(CalcError::Overflow)
    }
}

pub fn evaluate(input: &str) -> Result<f64, CalcError> {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    let [num_1, operator, num_2] = tokens.as_slice() else {
        return Err(CalcError::InvalidFormat);
    };

    calculate(
        parse_number(num_1)?,
        parse_number(num_2)?,
        operator.parse()?,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_operations() {
        assert_eq!(evaluate("3 + 4"), Ok(7.0));
        assert_eq!(evaluate("3 - 4"), Ok(-1.0));
        assert_eq!(evaluate("3 * 4"), Ok(12.0));
        assert_eq!(evaluate("12 / 4"), Ok(3.0));
    }

    #[test]
    fn handles_negatives_and_decimals() {
        assert_eq!(evaluate("-3 + 1.5"), Ok(-1.5));
        assert_eq!(evaluate("2 - -3"), Ok(5.0));
    }

    #[test]
    fn tolerates_extra_whitespace() {
        assert_eq!(evaluate(" 3  *  12 "), Ok(36.0));
    }

    #[test]
    fn rejects_divide_by_zero() {
        assert_eq!(evaluate("5 / 0"), Err(CalcError::DivideByZero));
        assert_eq!(evaluate("0 / 0"), Err(CalcError::DivideByZero));
        assert_eq!(evaluate("5 / -0"), Err(CalcError::DivideByZero));
    }

    #[test]
    fn rejects_invalid_numbers() {
        assert_eq!(
            evaluate("abc + 1"),
            Err(CalcError::InvalidNumber("abc".into()))
        );
        assert_eq!(
            evaluate("1 + inf"),
            Err(CalcError::InvalidNumber("inf".into()))
        );
        assert_eq!(
            evaluate("nan * 2"),
            Err(CalcError::InvalidNumber("nan".into()))
        );
    }

    #[test]
    fn rejects_invalid_operators() {
        assert_eq!(
            evaluate("6 ^ 2"),
            Err(CalcError::InvalidOperator("^".into()))
        );
        assert_eq!(
            evaluate("6 ** 2"),
            Err(CalcError::InvalidOperator("**".into()))
        );
    }

    #[test]
    fn rejects_wrong_token_count() {
        assert_eq!(evaluate(""), Err(CalcError::InvalidFormat));
        assert_eq!(evaluate("1 +"), Err(CalcError::InvalidFormat));
        assert_eq!(evaluate("1 + 2 + 3"), Err(CalcError::InvalidFormat));
    }

    #[test]
    fn detects_overflow() {
        assert_eq!(evaluate("1e308 * 10"), Err(CalcError::Overflow));
    }

    #[test]
    fn operator_parsing() {
        assert_eq!("+".parse::<Operator>(), Ok(Operator::Add));
        assert!("%".parse::<Operator>().is_err());
    }
}
