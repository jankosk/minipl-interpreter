use std::env;
use std::fs;

mod lexer;
mod token;
mod ast;
mod parser;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("Missing file path argument");
    let file = fs::read_to_string(file_path).unwrap();
    let file = file.trim();

    let lexer = Lexer::new(file);
    let mut parser = Parser::new(lexer);

    let statements = match parser.parse_program() {
        Ok(program) => program.statements,
        Err(err) => panic!("Failed with error: {:?}", err)
    };

    for statement in statements {
        println!("{}", statement);
    }
}
