use crate::ast::{Program, Statement, Expression, BinaryOperator};
use crate::lexer::Lexer;
use crate::token::Token;

pub enum ParseError {
    UnexpectedToken(Token),
}

pub type ParseResult<T> = Result<T, ParseError>;

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token::EOF,
            peek_token: Token::EOF,
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    pub fn parse_program(&mut self) -> ParseResult<Program> {
        let mut statements: Vec<Statement> = Vec::new();
        while self.current_token != Token::EOF {
            let statement = self.parse_statement()?;
            statements.push(statement);
            self.next_token();
        }
        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> ParseResult<Statement> {
        match self.get_current_token() {
            Token::Identifier(_) => self.parse_assignment(),
            Token::Var => self.parse_assignment(),
            other => Err(ParseError::UnexpectedToken(other))
        }
    }

    fn parse_assignment(&mut self) -> ParseResult<Statement> {
        if self.current_token == Token::Var {
            self.next_token();
        }

        let identifier = match self.current_token.clone() {
            Token::Identifier(id) => id,
            _ => return Err(ParseError::UnexpectedToken(self.get_current_token()))
        };

        self.expect_token(Token::Assign, ParseError::UnexpectedToken)?;
        self.next_token();

        let exp = self.parse_expression()?;
        
        Ok(Statement::Assignment(identifier, exp))
    }

    fn parse_expression(&mut self) -> ParseResult<Expression> {
        let token = self.get_current_token();
        let expression = match token {
            Token::Identifier(id) => Expression::Identifier(id),
            Token::IntegerConstant(value) => {
                let constant = value.parse::<i32>().unwrap();
                match self.peek_token.clone() {
                    Token::SemiColon => Expression::IntegerConstant(constant),
                    _ => self.parse_binary(Expression::IntegerConstant(constant))?,
                }
            },
            _ => return Err(ParseError::UnexpectedToken(token))
        };
        Ok(expression)
    }

    fn parse_binary(&mut self, left_exp: Expression) -> ParseResult<Expression> {
        let op = match self.get_peek_token() {
            Token::Plus => BinaryOperator::Plus,
            Token::Minus => BinaryOperator::Minus,
            invalid => return Err(ParseError::UnexpectedToken(invalid))
        };
        self.next_token(); //current token is an operator
        self.next_token();
        let right_exp = self.parse_expression()?;
        let exp = Expression::Binary(Box::new(left_exp), op, Box::new(right_exp));
        Ok(exp)
    }

    fn next_token(&mut self) {
        let next = self.peek_token.clone();
        self.current_token = next;
        self.peek_token = self.lexer.get_next_token();
    }

    fn get_current_token(&mut self) -> Token {
        self.current_token.clone()
    }

    fn get_peek_token(&mut self) -> Token {
        self.peek_token.clone()
    }

    fn expect_token(
        &mut self,
        expected: Token,
        err: fn(Token) -> ParseError
    ) -> ParseResult<()> {
        if expected == self.current_token {
            Ok(())
        } else {
            Err(err(self.get_current_token()))
        }
    }
}