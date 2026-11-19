use std::fmt;

use crate::{calculator::Operator, error::CalcError};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Number(f64),
    Op(Operator),
    LParen,
    RParen,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Number(n) => write!(f, "{n}"),
            Token::Op(op) => write!(f, "{op}"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
        }
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, CalcError> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else if c.is_ascii_digit() || c == '.' {
            let mut text = String::new();
            while let Some(&d) = chars.peek() {
                if d.is_ascii_digit() || d == '.' {
                    text.push(d);
                    chars.next();
                } else {
                    break;
                }
            }
            tokens.push(Token::Number(parse_number(&text)?));
        } else {
            match c {
                '(' => {
                    tokens.push(Token::LParen);
                    chars.next();
                }
                ')' => {
                    tokens.push(Token::RParen);
                    chars.next();
                }
                '+' | '-' | '*' | '/' => {
                    tokens.push(Token::Op(c.to_string().parse()?));
                    chars.next();
                }
                _ => return Err(CalcError::InvalidCharacter(c)),
            }
        }
    }

    Ok(tokens)
}

fn parse_number(input: &str) -> Result<f64, CalcError> {
    input
        .trim()
        .parse::<f64>()
        .ok()
        .filter(|n| n.is_finite())
        .ok_or_else(|| CalcError::InvalidNumber(input.trim().to_string()))
}

#[cfg(test)]
mod tests {
    use crate::parser::evaluate;

    use super::*;

    #[test]
    fn handles_complex_tokenization() {
        assert_eq!(
            tokenize("12+3.5*(2-1)"),
            Ok(vec![
                Token::Number(12.0),
                Token::Op(Operator::Add),
                Token::Number(3.5),
                Token::Op(Operator::Multiply),
                Token::LParen,
                Token::Number(2.0),
                Token::Op(Operator::Subtract),
                Token::Number(1.0),
                Token::RParen,
            ])
        );
    }

    #[test]
    fn rejects_invalid_characters() {
        assert_eq!(evaluate("abc + 1"), Err(CalcError::InvalidCharacter('a')));
        assert_eq!(evaluate("1 + inf"), Err(CalcError::InvalidCharacter('i')));
        assert_eq!(evaluate("nan * 2"), Err(CalcError::InvalidCharacter('n')));
        assert_eq!(evaluate("6 ^ 2"), Err(CalcError::InvalidCharacter('^')));
    }

    #[test]
    fn rejects_scientific_notation() {
        assert_eq!(
            evaluate("1e308 * 10"),
            Err(CalcError::InvalidCharacter('e'))
        );
    }
}
