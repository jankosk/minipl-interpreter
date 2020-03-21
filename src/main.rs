use std::env;
use std::fs;
use std::process;

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
    let file_path = match args.get(1) {
        Some(path) => path,
        None => {
            println!("Missing file path argument!");
            process::exit(1);
        }
    };
    let file = match fs::read_to_string(file_path) {
        Ok(file) => file,
        _ => {
            println!("File {} not found!", file_path);
            process::exit(1);
        }
    };

    let lexer = Lexer::new(file);
    let mut parser = Parser::new(lexer);

    let program = match parser.parse_program() {
        Ok(program) => program,
        Err(err) => {
            println!("Syntax Error: {:?}", err);
            process::exit(1);
        }
    };

    let mut evaluator = Evaluator::new(program);
    match evaluator.evaluate_program() {
        Ok(_) => {
            println!("\nSuccess!");
            process::exit(0);
        }
        Err(err) => {
            println!("\nFailed with error: {:?}", err);
            process::exit(1);
        }
    }
}
