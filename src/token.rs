use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    IntegerConstant(String),
    StringValue(String),
    Boolean(String),
    Plus,
    Minus,
    Assign,
    Var,
    Print,
    Colon,
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
            Token::Boolean(b) => b,
            Token::StringValue(s) => s,
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Assign => ":=",
            Token::Var => "var",
            Token::Print => "print",
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
        "true" => Token::Boolean(lexeme.to_string()),
        "false" => Token::Boolean(lexeme.to_string()),
        "var" => Token::Var,
        "print" => Token::Print,
        "bool" => Token::BooleanType,
        "string" => Token::StringType,
        "int" => Token::IntegerType,
        id => Token::Identifier(String::from(id)),
    }
}
