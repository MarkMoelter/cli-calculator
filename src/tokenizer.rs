use crate::calculator::{CalcError, Operator, parse_number};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Number(f64),
    Op(Operator),
    LParen,
    RParen,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, CalcError> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() || c == '.' {
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

#[cfg(test)]
mod tests {
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
}
