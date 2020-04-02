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
    ExpectedLeftBracket(Token),
    ExpectedIn(Token),
    ExpectedDo(Token),
    ExpectedRange(Token),
    ExpectedFor(Token),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = |expected, token| format!("Expected {} got {}", expected, token);
        let output = match self {
            ParseError::UnexpectedToken(t) => format!("Unexpected token: {}", t),
            ParseError::ExpectedColon(t) => msg(":", t),
            ParseError::ExpectedTypeDefinition(t) => msg("type definition", t),
            ParseError::ExpectedLeftBracket(t) => msg("(", t),
            ParseError::ExpectedClosingBracket(t) => msg(")", t),
            ParseError::ExpectedAssignment(t) => msg(":=", t),
            ParseError::ExpectedOperand(t) => msg("operand", t),
            ParseError::ExpectedSemiColon(t) => msg(";", t),
            ParseError::ExpectedIn(t) => msg("in keyword", t),
            ParseError::ExpectedDo(t) => msg("do keyword", t),
            ParseError::ExpectedRange(t) => msg("..", t),
            ParseError::ExpectedFor(t) => msg("for keyword", t),
            ParseError::ExpectedIdentifier(t) => msg("identifier", t),
        };
        write!(f, "{}", output)
    }
}

#[derive(Debug)]
pub enum EvalError {
    MismatchedTypes,
    UnsupportedOperation,
    VariableNotInitialized(String),
    VariableAlreadyInitialized(String),
    SyntaxError,
    IOError(String),
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = |err| format!("{}", err);
        let output = match self {
            EvalError::SyntaxError => msg("Syntax Error"),
            EvalError::MismatchedTypes => msg("Mismatched types"),
            EvalError::UnsupportedOperation => msg("Unsupported operation"),
            EvalError::VariableAlreadyInitialized(id) => {
                format!("Variable {} is already initialized", id)
            }
            EvalError::VariableNotInitialized(id) => format!("Variable {} not initialized", id),
            EvalError::IOError(err) => msg(err),
        };
        write!(f, "Failed with Error: {}", output)
    }
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
