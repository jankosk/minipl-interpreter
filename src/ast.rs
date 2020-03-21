use crate::utils::Type;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    VarInitialization(String, Type),
    NewAssignment(String, Type, Expression),
    Assignment(String, Expression),
    Print(Expression),
    For(String, Expression, Expression, Vec<Box<Statement>>),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::VarInitialization(id, type_def) => write!(f, "var {} : {};", id, type_def),
            Statement::NewAssignment(id, type_def, exp) => {
                write!(f, "var {} : {} := {};", id, type_def, exp)
            }
            Statement::Assignment(id, exp) => write!(f, "{} := {};", id, exp),
            Statement::Print(exp) => write!(f, "print {};", exp),
            Statement::For(id, exp1, exp2, stmts) => {
                let statements = stmts
                    .iter()
                    .map(|stmt| format!("\t{}", stmt))
                    .collect::<Vec<String>>()
                    .join("\n");
                write!(
                    f,
                    "for {} in {}..{} do\n{}\nend for",
                    id, exp1, exp2, statements
                )
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Identifier(String),
    IntegerConstant(i32),
    StringValue(String),
    Boolean(bool),
    Unary(UnaryOperator, Box<Expression>),
    Binary(Box<Expression>, BinaryOperator, Box<Expression>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Identifier(id) => write!(f, "{}", id),
            Expression::IntegerConstant(int) => write!(f, "{}", int),
            Expression::StringValue(s) => write!(f, "\"{}\"", s),
            Expression::Boolean(b) => write!(f, "{}", b),
            Expression::Unary(op, exp) => write!(f, "({} {})", op, exp),
            Expression::Binary(exp1, op, exp2) => write!(f, "({}, {}, {})", exp1, op, exp2),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    Not,
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnaryOperator::Not => write!(f, "!"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiplication,
    Division,
    And,
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            BinaryOperator::Plus => "+",
            BinaryOperator::Minus => "-",
            BinaryOperator::Multiplication => "*",
            BinaryOperator::Division => "/",
            BinaryOperator::And => "&",
        };
        write!(f, "{}", output)
    }
}
