use crate::token::{get_id_or_key_token, Token};

pub struct Lexer {
    position: usize,
    source: String,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.to_string(),
            position: 0,
            current_char: source.chars().next(),
        }
    }

    fn advance(&mut self) {
        self.position += 1;
        if self.source.len() > self.position {
            self.current_char = Some(self.source.as_bytes()[self.position] as char);
        } else {
            self.current_char = None;
        }
    }

    fn peek(&self) -> Option<char> {
        let pos = self.position + 1;
        if self.source.len() > pos {
            Some(self.source.as_bytes()[pos] as char)
        } else {
            None
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut lexeme = String::new();
        while self.peek() != None && self.peek().unwrap().is_alphabetic() {
            lexeme.push(self.current_char.unwrap());
            self.advance();
        }
        lexeme.push(self.current_char.unwrap());
        lexeme
    }

    fn read_integer(&mut self) -> String {
        let mut lexeme = String::new();
        while self.peek() != None && self.peek().unwrap().is_numeric() {
            lexeme.push(self.current_char.unwrap());
            self.advance();
        }
        lexeme.push(self.current_char.unwrap());
        lexeme
    }

    fn read_string(&mut self) -> String {
        let mut lexeme = String::new();
        while self.current_char != None && self.current_char.unwrap() != '"' {
            let current_char = self.current_char.unwrap();
            if current_char == '\\' {
                match self.peek() {
                    Some('"') => lexeme.push('\"'),
                    Some('\\') => lexeme.push('\\'),
                    Some('n') => lexeme.push('\n'),
                    Some('t') => lexeme.push('\t'),
                    _ => panic!("Unknown character escape!"),
                }
                self.advance();
            } else {
                lexeme.push(current_char);
            }
            self.advance();
        }
        lexeme
    }

    fn skip_whitespace(&mut self) {
        while self.current_char != None && self.current_char.unwrap().is_whitespace() {
            self.advance();
        }
    }

    pub fn get_next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.current_char {
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('*') => Token::Multiplication,
            Some('/') => Token::Division,
            Some(';') => Token::SemiColon,
            Some('&') => Token::And,
            Some('!') => Token::Not,
            Some('(') => Token::LeftBracket,
            Some(')') => Token::RightBracket,
            Some(':') => {
                if self.peek() == Some('=') {
                    self.advance();
                    Token::Assign
                } else {
                    Token::Colon
                }
            }
            Some('"') => {
                self.advance();
                Token::StringValue(self.read_string())
            }
            Some(ch) => {
                if ch.is_alphabetic() {
                    let lexeme = self.read_identifier();
                    get_id_or_key_token(&lexeme)
                } else if ch.is_numeric() {
                    let lexeme = self.read_integer();
                    Token::IntegerConstant(lexeme)
                } else {
                    panic!("Invalid character: {}", ch)
                }
            }
            None => Token::EOF,
        };

        self.advance();
        token
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::Token;

    #[test]
    fn lex_tokens() {
        let source = r#"
            var x : int := 1 + (2 - 1);
            x := 0;
            print x;
            var y : string := "a\"hello\"b\nworld\\";
            !true & false
        "#;
        let mut lexer = Lexer::new(&source);
        let expected_tokens = vec![
            Token::Var,
            Token::Identifier("x".to_string()),
            Token::Colon,
            Token::IntegerType,
            Token::Assign,
            Token::IntegerConstant("1".to_string()),
            Token::Plus,
            Token::LeftBracket,
            Token::IntegerConstant("2".to_string()),
            Token::Minus,
            Token::IntegerConstant("1".to_string()),
            Token::RightBracket,
            Token::SemiColon,
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::IntegerConstant("0".to_string()),
            Token::SemiColon,
            Token::Print,
            Token::Identifier("x".to_string()),
            Token::SemiColon,
            Token::Var,
            Token::Identifier("y".to_string()),
            Token::Colon,
            Token::StringType,
            Token::Assign,
            Token::StringValue("a\"hello\"b\nworld\\".to_string()),
            Token::SemiColon,
            Token::Not,
            Token::True,
            Token::And,
            Token::False,
            Token::EOF,
        ];
        for expected in expected_tokens {
            let token = lexer.get_next_token();
            assert_eq!(token, expected);
        }
    }
}
