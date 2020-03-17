use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Assignment(String, Expression),
    Print(Expression)
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Assignment(id, exp) => write!(f, "var {} := {};", id, exp),
            Statement::Print(exp) => write!(f, "print {};", exp),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Identifier(String),
    IntegerConstant(i32),
    Unary(UnaryOperator, Box<Expression>),
    Binary(Box<Expression>, BinaryOperator, Box<Expression>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Identifier(id) => write!(f, "{}", id),
            Expression::IntegerConstant(int) => write!(f, "{}", int),
            Expression::Unary(op, exp) => write!(f, "({} {})", op, exp),
            Expression::Binary(exp1, op, exp2) => write!(f, "({}, {}, {})", exp1, op, exp2),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Not,
    And,
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            UnaryOperator::Not => "!",
            UnaryOperator::And => "&",
        };
        write!(f, "{}", output)
    }
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiplication,
    Division,
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            BinaryOperator::Plus => "+",
            BinaryOperator::Minus => "-",
            BinaryOperator::Multiplication => "*",
            BinaryOperator::Division => "/",
        };
        write!(f, "{}", output)
    }
}