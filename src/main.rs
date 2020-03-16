use std::env;
use std::fs;

mod lexer;
mod token;
mod ast;
mod parser;

use lexer::Lexer;
use token::Token;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).unwrap();
    let file = fs::read_to_string(file_path).unwrap();
    let file = file.trim();

    println!("File: {}", file);

    let mut lexer = Lexer::new(file);
    loop {
        let token = lexer.get_next_token();
        println!("Token {}", token);
        if token == Token::EOF {
            break;
        }
    }
}
