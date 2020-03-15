use crate::token::{Token, lookup_identifier};

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

    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while self.current_char != None && self.current_char.unwrap().is_alphabetic() {
            identifier.push(self.current_char.unwrap());
            self.advance();
        }
        return identifier;
    }

    fn read_integer(&mut self) -> i32 {
        let mut result = String::new();
        while self.current_char != None && self.current_char.unwrap().is_numeric() {
            result.push(self.current_char.unwrap());
            self.advance();
        }
        return result.parse::<i32>().unwrap();
    }

    fn skip_whitespace(&mut self) {
        while self.current_char != None && self.current_char.unwrap().is_whitespace() {
            self.advance();
        }
    }

    fn peek(&self) -> char {
        let next = self.source.as_bytes()[self.position + 1] as char;
        return next;
    }

    pub fn get_next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.current_char {
            Some(ch) => match ch {
                '+' => Token::Plus,
                '-' => Token::Minus,
                ';' => Token::SemiColon,
                ':' => {
                    if self.peek() == '=' {
                        self.advance();
                        Token::Assign
                    } else {
                        Token::Colon
                    }
                }
                ch => {
                  if ch.is_alphabetic() {
                      let lexeme = self.read_identifier();
                      lookup_identifier(&lexeme)
                  } else if ch.is_numeric() {
                      let lexeme = self.read_integer();
                      Token::IntegerConstant(lexeme.to_string())
                  } else {
                      panic!("Invalid character: {}", ch)
                  }
                }
            },
            None => Token::EOF,
        };

        self.advance();
        token
    }
}
