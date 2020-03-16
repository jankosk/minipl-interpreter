use std::fmt;

pub struct Program {
    statements: Vec<Statement>,
}

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

pub enum Expression {
    Identifier(String),
    IntegerConstant(i32),
    Unary(UnaryOperator, Expression),
    Binary(Expression, BinaryOperator, Expression),
    Print(Expression),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Identifier(id) => write!(f, "{}", id),
            Expression::IntegerConstant(int) => write!(f, "{}", int),
            Expression::Unary(op, exp) => write!(f, "({} {})", op, exp),
            Expression::Binary(exp, op, exp) => write!(f, "({}, {}, {})", exp, op, exp),
        }
    }
}

pub enum UnaryOperator {
    Not,
    And,
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            UnaryOperator::Not => "!",
            UnaryOperator::And => "&",
        }
        write!(f, "{}", output);
    }
}

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
        }
        write!(f, "{}", output);
    }
}