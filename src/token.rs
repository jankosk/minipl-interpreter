use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    IntegerConstant(String),
    Plus,
    Minus,
    Assign,
    Var,
    Colon,
    SemiColon,
    EOF,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            Token::Identifier(name) => name,
            Token::IntegerConstant(constant) => constant,
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Assign => ":=",
            Token::Var => "var",
            Token::Colon => ":",
            Token::SemiColon => ";",
            Token::EOF => "EOF",
        };
        write!(f, "{}", output)
    }
}

pub fn lookup_identifier(lexeme: &str) -> Token {
    match lexeme {
        "var" => Token::Var,
        id => Token::Identifier(String::from(id)),
    }
}