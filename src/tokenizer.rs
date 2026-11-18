use crate::calculator::Operator;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Number(f64),
    Op(Operator),
    LParen,
    RParen,
}
