use std::{fmt, str::FromStr};

use crate::error::CalcError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl FromStr for Operator {
    type Err = CalcError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Subtract),
            "*" => Ok(Operator::Multiply),
            "/" => Ok(Operator::Divide),
            _ => Err(CalcError::InvalidToken(s.to_string())),
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
        };
        write!(f, "{symbol}")
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operator_parsing() {
        assert_eq!("+".parse::<Operator>(), Ok(Operator::Add));
        assert!("%".parse::<Operator>().is_err());
    }

    // calculator.rs
    #[test]
    fn detects_overflow() {
        assert_eq!(
            calculate(1e308, 10.0, Operator::Multiply),
            Err(CalcError::Overflow)
        );
    }
}
