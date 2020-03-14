use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).unwrap();
    let file = fs::read_to_string(file_path).unwrap();
    let file = file.trim();

    println!("File: {}", file);
}
