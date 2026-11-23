use crate::{
    calculator::{Operator, calculate},
    error::CalcError,
    tokenizer::{Token, tokenize},
};

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.pos).copied()
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn parse_expression(&mut self) -> Result<f64, CalcError> {
        let mut value = self.parse_term()?;
        while let Some(Token::Op(op @ (Operator::Add | Operator::Subtract))) = self.peek() {
            self.advance();
            let rhs = self.parse_term()?;
            value = calculate(value, rhs, op)?;
        }
        Ok(value)
    }

    fn parse_term(&mut self) -> Result<f64, CalcError> {
        let mut value = self.parse_factor()?;
        while let Some(Token::Op(op @ (Operator::Multiply | Operator::Divide))) = self.peek() {
            self.advance();
            let rhs = self.parse_factor()?;
            value = calculate(value, rhs, op)?;
        }
        Ok(value)
    }

    fn parse_factor(&mut self) -> Result<f64, CalcError> {
        match self.peek() {
            Some(Token::Number(n)) => {
                self.advance();
                Ok(n)
            }
            Some(Token::Op(Operator::Subtract)) => {
                self.advance();
                let value = self.parse_factor()?;
                Ok(-value)
            }
            Some(Token::LParen) => {
                self.advance();
                let value = self.parse_expression()?;
                match self.peek() {
                    Some(Token::RParen) => {
                        self.advance();
                        Ok(value)
                    }
                    Some(other) => Err(CalcError::UnexpectedToken(other.to_string())),
                    None => Err(CalcError::UnexpectedEnd),
                }
            }
            Some(other) => Err(CalcError::UnexpectedToken(other.to_string())),
            None => Err(CalcError::UnexpectedEnd),
        }
    }
}

pub fn evaluate(input: &str) -> Result<f64, CalcError> {
    let tokens = tokenize(input)?;
    let mut parser = Parser { tokens, pos: 0 };
    let value = parser.parse_expression()?;
    if let Some(extra) = parser.peek() {
        return Err(CalcError::UnexpectedToken(extra.to_string()));
    }
    Ok(value)
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
    fn detects_overflow() {
        let input = format!("1{} * 10", "0".repeat(308)); // 1e308 * 10
        assert_eq!(evaluate(&input), Err(CalcError::Overflow));
    }

    #[test]
    fn parse_basic_expressions() {
        assert_eq!(evaluate("6 + 3"), Ok(9.0));
    }

    #[test]
    fn handles_parentheses() {
        let tokens = vec![
            Token::LParen,
            Token::Number(6.0),
            Token::Op(Operator::Add),
            Token::Number(3.0),
            Token::RParen,
            Token::Op(Operator::Multiply),
            Token::Number(4.0),
        ];
        let mut parser = Parser { tokens, pos: 0 };
        assert_eq!(parser.parse_expression(), Ok(36.0))
    }

    #[test]
    fn handles_precedence() {
        assert_eq!(evaluate("2 + 3 * 4"), Ok(14.0));
        assert_eq!(evaluate("2 * 3 + 4"), Ok(10.0));
        assert_eq!(evaluate("2 * (3 + 4)"), Ok(14.0));
        assert_eq!(evaluate("(2 + 3) * 4"), Ok(20.0));
        assert_eq!(evaluate("10 - 3 - 2"), Ok(5.0)); // left associativity
        assert_eq!(evaluate("-(2 + 3)"), Ok(-5.0)); // unary minus on a group
    }

    #[test]
    fn rejects_unexpected_tokens() {
        // evaluate
        assert_eq!(
            evaluate("2 3"),
            Err(CalcError::UnexpectedToken("3".to_string()))
        );
        assert_eq!(
            evaluate("2 + 3)"),
            Err(CalcError::UnexpectedToken(")".to_string()))
        );

        // parser
        assert_eq!(evaluate("*"), Err(CalcError::UnexpectedToken("*".into())));
        assert_eq!(
            evaluate("(2 * 5 4"),
            Err(CalcError::UnexpectedToken("4".into()))
        );
    }

    #[test]
    fn rejects_incomplete_input_on_parse_factor() {
        assert_eq!(evaluate(""), Err(CalcError::UnexpectedEnd)); // outer
        assert_eq!(evaluate("1 +"), Err(CalcError::UnexpectedEnd)); // outer
        assert_eq!(evaluate("("), Err(CalcError::UnexpectedEnd)); // outer, via the recursion
        assert_eq!(evaluate("(2 + 3"), Err(CalcError::UnexpectedEnd)); // inner
    }
}
