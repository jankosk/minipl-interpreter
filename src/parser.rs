use crate::ast::{BinaryOperator, Expression, Program, Statement, UnaryOperator};
use crate::lexer::Lexer;
use crate::token::Token;
use crate::utils::{ParseError, Type};

type ParseResult<T> = Result<T, ParseError>;

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<ParseError>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token::EOF,
            peek_token: Token::EOF,
            errors: Vec::new(),
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    pub fn get_errors(&self) -> &[ParseError] {
        &self.errors
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements: Vec<Statement> = Vec::new();
        while self.current_token != Token::EOF {
            match self.parse_statement() {
                Ok(stmt) => {
                    statements.push(stmt);
                }
                Err(err) => {
                    self.errors.push(err);
                }
            };
            self.next_token();
        }
        Program { statements }
    }

    fn parse_statement(&mut self) -> ParseResult<Statement> {
        match self.get_current_token() {
            Token::Identifier(_) => self.parse_assignment(),
            Token::Var => self.parse_new_assignment(),
            Token::For => self.parse_for(),
            Token::Assert => self.parse_assert(),
            Token::Print => {
                self.next_token();
                let exp = self.parse_expression()?;
                self.next_token();
                self.expect_current_token(Token::SemiColon, ParseError::ExpectedSemiColon)?;
                Ok(Statement::Print(exp))
            }
            Token::Read => {
                self.next_token();
                let identifier = self.parse_identifier()?;
                self.next_token();
                self.expect_current_token(Token::SemiColon, ParseError::ExpectedSemiColon)?;
                Ok(Statement::Read(identifier))
            }
            other => Err(ParseError::UnexpectedToken(other)),
        }
    }

    fn parse_for(&mut self) -> ParseResult<Statement> {
        self.next_token();
        let identifier = self.parse_identifier()?;
        self.next_token();
        self.expect_and_advance(Token::In, ParseError::ExpectedIn)?;

        let exp1 = self.parse_expression()?;
        self.next_token();
        self.expect_and_advance(Token::Range, ParseError::ExpectedRange)?;

        let exp2 = self.parse_expression()?;
        self.next_token();

        self.expect_and_advance(Token::Do, ParseError::ExpectedDo)?;

        let mut stmts: Vec<Box<Statement>> = Vec::new();
        while self.current_token != Token::End {
            let stmt = self.parse_statement()?;
            stmts.push(Box::new(stmt));
            self.next_token();
        }
        self.next_token();

        self.expect_and_advance(Token::For, ParseError::ExpectedFor)?;
        self.expect_current_token(Token::SemiColon, ParseError::ExpectedSemiColon)?;

        Ok(Statement::For(identifier, exp1, exp2, stmts))
    }

    fn parse_assignment(&mut self) -> ParseResult<Statement> {
        let identifier = self.parse_identifier()?;
        self.next_token();

        self.expect_and_advance(Token::Assign, ParseError::ExpectedAssignment)?;

        let exp = self.parse_expression()?;
        self.next_token();
        self.expect_current_token(Token::SemiColon, ParseError::ExpectedSemiColon)?;

        Ok(Statement::Assignment(identifier, exp))
    }

    fn parse_new_assignment(&mut self) -> ParseResult<Statement> {
        self.next_token();
        let identifier = self.parse_identifier()?;
        self.next_token();

        self.expect_and_advance(Token::Colon, ParseError::ExpectedColon)?;

        let type_def = match self.get_current_token() {
            Token::BooleanType => Type::Boolean,
            Token::IntegerType => Type::Integer,
            Token::StringType => Type::String,
            invalid => return Err(ParseError::ExpectedTypeDefinition(invalid)),
        };
        self.next_token();

        if self.current_token == Token::SemiColon {
            return Ok(Statement::VarInitialization(identifier, type_def));
        }

        self.expect_and_advance(Token::Assign, ParseError::ExpectedAssignment)?;
        let exp = self.parse_expression()?;
        self.next_token();
        self.expect_current_token(Token::SemiColon, ParseError::ExpectedSemiColon)?;
        Ok(Statement::NewAssignment(identifier, type_def, exp))
    }

    fn parse_assert(&mut self) -> ParseResult<Statement> {
        self.next_token();
        self.expect_and_advance(Token::LeftBracket, ParseError::ExpectedLeftBracket)?;
        let exp = self.parse_expression()?;
        self.next_token();
        self.expect_current_token(Token::RightBracket, ParseError::ExpectedClosingBracket)?;
        self.next_token();
        self.expect_current_token(Token::SemiColon, ParseError::ExpectedSemiColon)?;
        Ok(Statement::Assert(exp))
    }

    fn parse_expression(&mut self) -> ParseResult<Expression> {
        match self.get_current_token() {
            Token::Not => self.parse_unary(UnaryOperator::Not),
            _ => self.parse_binary(),
        }
    }

    fn parse_binary(&mut self) -> ParseResult<Expression> {
        let left = self.parse_operand()?;
        if self.is_end_of_exp() {
            return Ok(left)
        }
        self.next_token();
        let op = self.parse_op()?;
        self.next_token();
        let right = self.parse_operand()?;
        let exp = Expression::Binary(Box::new(left), op, Box::new(right));
        Ok(exp)
    }

    fn parse_unary(&mut self, op: UnaryOperator) -> ParseResult<Expression> {
        self.next_token();
        let exp = self.parse_binary()?;
        let exp = Expression::Unary(op, Box::new(exp));
        Ok(exp)
    }

    fn parse_operand(&mut self) -> ParseResult<Expression> {
        let operand = match self.get_current_token() {
            Token::Identifier(id) => Expression::Identifier(id),
            Token::IntegerConstant(int) => Expression::IntegerConstant(int.parse::<i32>().unwrap()),
            Token::StringValue(string) => Expression::StringValue(string),
            Token::True => Expression::Boolean(true),
            Token::False => Expression::Boolean(false),
            Token::LeftBracket => {
                self.next_token();
                let exp = self.parse_expression()?;
                self.next_token();
                self.expect_current_token(Token::RightBracket, ParseError::ExpectedClosingBracket)?;
                exp
            }
            invalid => return Err(ParseError::ExpectedOperand(invalid)),
        };
        Ok(operand)
    }

    fn parse_op(&mut self) -> ParseResult<BinaryOperator> {
        match self.get_current_token() {
            Token::Plus => Ok(BinaryOperator::Plus),
            Token::Minus => Ok(BinaryOperator::Minus),
            Token::Multiplication => Ok(BinaryOperator::Multiplication),
            Token::Division => Ok(BinaryOperator::Division),
            Token::Equals => Ok(BinaryOperator::Equals),
            Token::LessThan => Ok(BinaryOperator::LessThan),
            Token::GreaterThan => Ok(BinaryOperator::GreaterThan),
            Token::And => Ok(BinaryOperator::And),
            invalid => Err(ParseError::UnexpectedToken(invalid)),
        }        
    }

    fn parse_identifier(&mut self) -> ParseResult<String> {
        match self.get_current_token() {
            Token::Identifier(id) => Ok(id.to_string()),
            invalid => Err(ParseError::ExpectedIdentifier(invalid.clone())),
        }
    }

    fn is_end_of_exp(&self) -> bool {
        match self.peek_token {
            Token::SemiColon => true,
            Token::RightBracket => true,
            Token::Range => true,
            Token::Do => true,
            Token::End => true,
            _ => false
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

    fn expect_and_advance(
        &mut self,
        expected: Token,
        err: fn(Token) -> ParseError,
    ) -> ParseResult<()> {
        self.expect_current_token(expected, err)?;
        self.next_token();
        Ok(())
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
    use crate::ast::{BinaryOperator, Expression, Statement, UnaryOperator};
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::token::Token;
    use crate::utils::{ParseError, Type};

    #[test]
    fn parse_assignment() -> Result<(), ParseError> {
        let source = r#"
            var x : int := 1 + 2;
            x := x - 1;
            var yY_1 : string := "hello";
            var Zz2_ : bool;
        "#;
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let expected = vec![
            Statement::NewAssignment(
                "x".to_string(),
                Type::Integer,
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
            Statement::NewAssignment(
                "yY_1".to_string(),
                Type::String,
                Expression::StringValue("hello".to_string()),
            ),
            Statement::VarInitialization("Zz2_".to_string(), Type::Boolean),
        ];
        assert_eq!(program.statements, expected);
        Ok(())
    }

    #[test]
    fn parse_print() -> Result<(), ParseError> {
        let source = r#"
            print "hello";
            print (1 + 2);
            print !true;
            print 1 + (2 / (3 * 2));
            print 1 = 1;
        "#;
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        let expected = vec![
            Statement::Print(Expression::StringValue("hello".to_string())),
            Statement::Print(Expression::Binary(
                Box::new(Expression::IntegerConstant(1)),
                BinaryOperator::Plus,
                Box::new(Expression::IntegerConstant(2)),
            )),
            Statement::Print(Expression::Unary(
                UnaryOperator::Not,
                Box::new(Expression::Boolean(true)),
            )),
            Statement::Print(Expression::Binary(
                Box::new(Expression::IntegerConstant(1)),
                BinaryOperator::Plus,
                Box::new(Expression::Binary(
                    Box::new(Expression::IntegerConstant(2)),
                    BinaryOperator::Division,
                    Box::new(Expression::Binary(
                        Box::new(Expression::IntegerConstant(3)),
                        BinaryOperator::Multiplication,
                        Box::new(Expression::IntegerConstant(2)),
                    )),
                )),
            )),
            Statement::Print(Expression::Binary(
                Box::new(Expression::IntegerConstant(1)),
                BinaryOperator::Equals,
                Box::new(Expression::IntegerConstant(1)),
            )),
        ];
        assert_eq!(program.statements, expected);
        Ok(())
    }

    #[test]
    fn parse_for() -> Result<(), ParseError> {
        let source = r#"
            for x in 1..5 do
                print x;
                print "hello";
            end for;
        "#;
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let expected = vec![Statement::For(
            "x".to_string(),
            Expression::IntegerConstant(1),
            Expression::IntegerConstant(5),
            vec![
                Box::new(Statement::Print(Expression::Identifier("x".to_string()))),
                Box::new(Statement::Print(Expression::StringValue(
                    "hello".to_string(),
                ))),
            ],
        )];
        println!("{}", expected[0]);
        assert_eq!(program.statements, expected);
        Ok(())
    }

    #[test]
    fn report_error() {
        let source = "print 1);";
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer);
        parser.parse_program();
        let errors = parser.get_errors();
        assert_eq!(
            true,
            errors.contains(&ParseError::ExpectedSemiColon(Token::RightBracket))
        );
    }
}
