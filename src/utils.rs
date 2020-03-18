use std::fmt;
use crate::token::Token;

pub enum ParseError {
    UnexpectedToken(Token),
}

#[derive(Debug)]
pub enum EvalError {
    MismatchedTypes,
    UnsupportedOperation,
    VariableNotInitialized(String),
    SyntaxError,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Bool(bool),
    String(String),
    Integer(i32),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Bool(b) => write!(f, "{}", b),
            Type::Integer(int) => write!(f, "{}", int),
            Type::String(s) => write!(f, "{}", s),
        }
    }
}