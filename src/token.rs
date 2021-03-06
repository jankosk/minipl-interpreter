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
    Equals,
    LessThan,
    GreaterThan,
    Assign,
    Var,
    Print,
    Colon,
    True,
    False,
    Read,
    And,
    Not,
    For,
    In,
    Do,
    End,
    Range,
    Assert,
    LeftBracket,
    RightBracket,
    SemiColon,
    IntegerType,
    StringType,
    BooleanType,
    EOF,
    Illegal,
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
            Token::And => "&",
            Token::Not => "!",
            Token::LeftBracket => "(",
            Token::RightBracket => ")",
            Token::Colon => ":",
            Token::SemiColon => ";",
            Token::Equals => "=",
            Token::LessThan => "<",
            Token::GreaterThan => ">",
            Token::Assign => ":=",
            Token::Var => "var",
            Token::Print => "print",
            Token::For => "for",
            Token::In => "in",
            Token::Do => "do",
            Token::End => "end",
            Token::Range => "..",
            Token::Assert => "assert",
            Token::Read => "read",
            Token::True => "true",
            Token::False => "false",
            Token::BooleanType => "bool",
            Token::StringType => "string",
            Token::IntegerType => "int",
            Token::EOF => "EOF",
            Token::Illegal => "Illegal Token!",
        };
        write!(f, "{}", output)
    }
}

pub fn get_id_or_key_token(lexeme: &str) -> Token {
    match lexeme {
        "for" => Token::For,
        "in" => Token::In,
        "do" => Token::Do,
        "end" => Token::End,
        "true" => Token::True,
        "false" => Token::False,
        "var" => Token::Var,
        "print" => Token::Print,
        "bool" => Token::BooleanType,
        "string" => Token::StringType,
        "int" => Token::IntegerType,
        "assert" => Token::Assert,
        "read" => Token::Read,
        id => Token::Identifier(String::from(id)),
    }
}
