use crate::token::Token;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedToken(Token),
    ExpectedColon(Token),
    ExpectedTypeDefinition(Token),
    ExpectedAssignment(Token),
    ExpectedIdentifier(Token),
    ExpectedOperand(Token),
    ExpectedSemiColon(Token),
    ExpectedClosingBracket(Token),
}

#[derive(Debug)]
pub enum EvalError {
    MismatchedTypes,
    UnsupportedOperation,
    VariableNotInitialized(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Boolean,
    String,
    Integer,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Boolean => write!(f, "bool"),
            Type::String => write!(f, "string"),
            Type::Integer => write!(f, "int"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Bool(bool),
    String(String),
    Integer(i32),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{}", b),
            Value::Integer(int) => write!(f, "{}", int),
            Value::String(s) => write!(f, "{}", s),
        }
    }
}
