use crate::ast::{Program, Statement, Expression};
use crate::lexer::Lexer;
use crate::token::Token;

enum ParseError {
    UnexpectedToken(Token),
}

type ParseResult<T> = Result<T, ParseError>;

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token::EOF,
        };
        parser.next_token();
        parser
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements: Vec<Statement> = Vec::new();
        while self.current_token != Token::EOF {
            let statement = self.parse_statement();
            match statement {
                Ok(stmt) => statements.push(stmt),
                Err(err) => panic!(err),
            }
            self.next_token();
        }

        Program { statements }
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
            Token::IntegerConstant(value) => Expression::IntegerConstant(value.parse().unwrap()),
            Token::Identifier(id) => Expression::Identifier(id),
            _ => return Err(ParseError::UnexpectedToken(token))
        };
        Ok(expression)
    }

    fn next_token(&mut self) {
        let token = self.lexer.get_next_token();
        self.current_token = token;
    }

    fn get_current_token(&mut self) -> Token {
        self.current_token.clone()
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