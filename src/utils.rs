use std::fmt;
use crate::token::Token;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
    ExpectedColon(Token),
    ExpectedTypeDefinition(Token),
    ExpectedAssignment(Token),
    ExpectedIdentifier(Token),
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

pub fn is_type(token: &Token) -> bool {
    match token {
        Token::BooleanType => true,
        Token::StringType => true,
        Token::IntegerType => true,
        _ => false,
    }
}