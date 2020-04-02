extern crate regex;

mod ast;
mod evaluator;
mod lexer;
mod parser;
mod token;
mod utils;

use evaluator::Evaluator;
use lexer::Lexer;
use parser::Parser;
use std::env;
use std::fs;
use std::process;
use utils::EvalError;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = match args.get(1) {
        Some(path) => path,
        None => "",
    };
    let file = match fs::read_to_string(file_path) {
        Ok(file) => file,
        _ => {
            println!("File {} not found!", file_path);
            process::exit(1);
        }
    };

    match interpret(file) {
        Ok(_) => {
            println!("\nSuccess!");
            process::exit(0);
        }
        Err(err) => {
            println!("\n{}", err);
            process::exit(1);
        }
    }
}

fn interpret(file: String) -> Result<(), EvalError> {
    let lexer = Lexer::new(file);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    let syntax_errors = parser.get_errors();
    match syntax_errors.is_empty() {
        false => {
            for err in syntax_errors {
                println!("{}", err);
            }
            Err(EvalError::SyntaxError)
        }
        _ => {
            let mut evaluator = Evaluator::new(program);
            evaluator.evaluate_program()
        }
    }
}
