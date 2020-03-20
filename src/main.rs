use std::env;
use std::fs;

mod ast;
mod evaluator;
mod lexer;
mod parser;
mod token;
mod utils;

use evaluator::Evaluator;
use lexer::Lexer;
use parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("Missing file path argument");
    let file = fs::read_to_string(file_path).unwrap();
    let file = file.trim();

    let lexer = Lexer::new(file);
    let mut parser = Parser::new(lexer);

    let program = match parser.parse_program() {
        Ok(program) => program,
        Err(err) => panic!("Syntax Error: {:?}", err),
    };

    let mut evaluator = Evaluator::new(program);
    match evaluator.evaluate_program() {
        Ok(_) => println!("Success!"),
        Err(err) => println!("Failed with error: {:?}", err),
    }
}
