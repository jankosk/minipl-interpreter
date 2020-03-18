use crate::ast::{BinaryOperator, Expression, Program, Statement};
use crate::lexer::Lexer;
use crate::token::Token;
use crate::utils::{is_type, ParseError};

type ParseResult<T> = Result<T, ParseError>;

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
            Token::Print => {
                self.next_token();
                let exp = self.parse_expression()?;
                Ok(Statement::Print(exp))
            }
            other => Err(ParseError::UnexpectedToken(other)),
        }
    }

    fn parse_assignment(&mut self) -> ParseResult<Statement> {
        let identifier;
        if self.current_token == Token::Var {
            self.next_token();
            identifier = self.parse_identifier()?;
            self.next_token();
            self.expect_current_token(Token::Colon, ParseError::ExpectedColon)?;
            self.next_token();
            match is_type(&self.current_token) {
                true => self.next_token(),
                _ => return Err(ParseError::ExpectedTypeDefinition(self.get_current_token())),
            }
        } else {
            identifier = self.parse_identifier()?;
            self.next_token();
        }
        self.expect_current_token(Token::Assign, ParseError::ExpectedAssignment)?;
        self.next_token();

        let exp = self.parse_expression()?;
        Ok(Statement::Assignment(identifier, exp))
    }

    fn parse_expression(&mut self) -> ParseResult<Expression> {
        let token = self.get_current_token();
        let expression = match token {
            Token::Identifier(id) => {
                let left_exp = Expression::Identifier(id);
                self.next_token();
                self.parse_operator(left_exp)?
            }
            Token::IntegerConstant(value) => {
                let constant = value.parse::<i32>().unwrap();
                let left_exp = Expression::IntegerConstant(constant);
                self.next_token();
                self.parse_operator(left_exp)?
            }
            Token::StringValue(value) => {
                let left_exp = Expression::StringValue(value);
                self.next_token();
                self.parse_operator(left_exp)?
            }
            Token::True => {
                let left_exp = Expression::Boolean(true);
                self.next_token();
                self.parse_operator(left_exp)?
            }
            Token::False => {
                let left_exp = Expression::Boolean(false);
                self.next_token();
                self.parse_operator(left_exp)?
            }
            _ => return Err(ParseError::UnexpectedToken(token)),
        };
        Ok(expression)
    }

    fn parse_operator(&mut self, left_exp: Expression) -> ParseResult<Expression> {
        match self.get_current_token() {
            Token::SemiColon => Ok(left_exp),
            Token::Plus => self.parse_binary(left_exp, BinaryOperator::Plus),
            Token::Minus => self.parse_binary(left_exp, BinaryOperator::Minus),
            invalid => return Err(ParseError::UnexpectedToken(invalid)),
        }
    }

    fn parse_binary(
        &mut self,
        left_exp: Expression,
        op: BinaryOperator,
    ) -> ParseResult<Expression> {
        self.next_token();
        let right_exp = self.parse_expression()?;
        let exp = Expression::Binary(Box::new(left_exp), op, Box::new(right_exp));
        Ok(exp)
    }

    fn parse_identifier(&mut self) -> ParseResult<String> {
        match self.get_current_token() {
            Token::Identifier(id) => Ok(id.to_string()),
            invalid => Err(ParseError::ExpectedIdentifier(invalid.clone())),
        }
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

    fn expect_current_token(
        &mut self,
        expected: Token,
        err: fn(Token) -> ParseError,
    ) -> ParseResult<()> {
        if expected == self.current_token {
            Ok(())
        } else {
            Err(err(self.get_current_token()))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{BinaryOperator, Expression, Statement};
    use crate::lexer::Lexer;
    use crate::parser::{ParseError, Parser};

    #[test]
    fn parse_assignment() -> Result<(), ParseError> {
        let source = r#"
            var x : int := 1 + 2;
            x := x - 1;
            var y : string := "hello";
            var z : bool := true;
        "#;
        let lexer = Lexer::new(&source);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program()?;
        let expected = vec![
            Statement::Assignment(
                "x".to_string(),
                Expression::Binary(
                    Box::new(Expression::IntegerConstant(1)),
                    BinaryOperator::Plus,
                    Box::new(Expression::IntegerConstant(2)),
                ),
            ),
            Statement::Assignment(
                "x".to_string(),
                Expression::Binary(
                    Box::new(Expression::Identifier("x".to_string())),
                    BinaryOperator::Minus,
                    Box::new(Expression::IntegerConstant(1)),
                ),
            ),
            Statement::Assignment(
                "y".to_string(),
                Expression::StringValue("hello".to_string()),
            ),
            Statement::Assignment("z".to_string(), Expression::Boolean(true)),
        ];
        assert_eq!(program.statements, expected);
        Ok(())
    }

    #[test]
    fn parse_print() -> Result<(), ParseError> {
        let source = "print 1; print 1 + 2;";
        let lexer = Lexer::new(&source);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program()?;
        let expected = vec![
            Statement::Print(Expression::IntegerConstant(1)),
            Statement::Print(Expression::Binary(
                Box::new(Expression::IntegerConstant(1)),
                BinaryOperator::Plus,
                Box::new(Expression::IntegerConstant(2)),
            )),
        ];
        assert_eq!(program.statements, expected);
        Ok(())
    }
}
