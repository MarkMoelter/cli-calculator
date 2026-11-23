use std::fmt;

use crate::error::CalcError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
            Operator::Modulo => "%",
        };
        write!(f, "{symbol}")
    }
}

impl Operator {
    pub fn from_char(c: char) -> Option<Operator> {
        match c {
            '+' => Some(Operator::Add),
            '-' => Some(Operator::Subtract),
            '*' => Some(Operator::Multiply),
            '/' => Some(Operator::Divide),
            '%' => Some(Operator::Modulo),
            _ => None,
        }
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
        Operator::Modulo => num_1 % num_2,
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
    fn detects_overflow() {
        assert_eq!(
            calculate(1e308, 10.0, Operator::Multiply),
            Err(CalcError::Overflow)
        );
    }

    #[test]
    fn operator_display() {
        assert_eq!(Operator::Add.to_string(), "+");
        assert_eq!(Operator::Subtract.to_string(), "-");
        assert_eq!(Operator::Multiply.to_string(), "*");
        assert_eq!(Operator::Divide.to_string(), "/");
        assert_eq!(Operator::Modulo.to_string(), "%");
    }

    #[test]
    fn operator_from_char() {
        assert_eq!(Operator::from_char('+'), Some(Operator::Add));
        assert_eq!(Operator::from_char('/'), Some(Operator::Divide));
        assert_eq!(Operator::from_char('x'), None);
    }

    #[test]
    fn basic_arithmetic() {
        assert_eq!(calculate(3.0, 2.0, Operator::Add), Ok(5.0));
        assert_eq!(calculate(3.0, 2.0, Operator::Subtract), Ok(1.0));
        assert_eq!(calculate(3.0, 2.0, Operator::Multiply), Ok(6.0));
        assert_eq!(calculate(3.0, 2.0, Operator::Divide), Ok(1.5));
        assert_eq!(calculate(12.0, 2.0, Operator::Modulo), Ok(0.0));
        assert_eq!(calculate(17.0, 12.0, Operator::Modulo), Ok(5.0));
    }
}
