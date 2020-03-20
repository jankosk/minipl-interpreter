use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    IntegerConstant(String),
    StringValue(String),
    Plus,
    Minus,
    Multiplication,
    Division,
    Assign,
    Var,
    Print,
    Colon,
    True,
    False,
    And,
    Not,
    LeftBracket,
    RightBracket,
    SemiColon,
    IntegerType,
    StringType,
    BooleanType,
    EOF,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            Token::Identifier(name) => name,
            Token::IntegerConstant(constant) => constant,
            Token::StringValue(s) => s,
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Multiplication => "*",
            Token::Division => "/",
            Token::Assign => ":=",
            Token::Var => "var",
            Token::Print => "print",
            Token::True => "true",
            Token::False => "false",
            Token::And => "&",
            Token::Not => "!",
            Token::LeftBracket => "(",
            Token::RightBracket => ")",
            Token::BooleanType => "bool",
            Token::StringType => "string",
            Token::IntegerType => "int",
            Token::Colon => ":",
            Token::SemiColon => ";",
            Token::EOF => "EOF",
        };
        write!(f, "{}", output)
    }
}

pub fn get_id_or_key_token(lexeme: &str) -> Token {
    match lexeme {
        "true" => Token::True,
        "false" => Token::False,
        "var" => Token::Var,
        "print" => Token::Print,
        "bool" => Token::BooleanType,
        "string" => Token::StringType,
        "int" => Token::IntegerType,
        id => Token::Identifier(String::from(id)),
    }
}
