use std::{env, fs};

use lexer::tokenize;
use parser::parse;

pub mod lexer;
pub mod parser;

fn main() {
    let sys_args = env::args().collect::<Vec<String>>();
    let contents = if sys_args.len() >= 2 {
        println!("Running {}", sys_args[1]);
        fs::read_to_string(sys_args[1].clone()).expect("Something went wrong reading the file")
    } else {
        println!("Running main.abf");
        fs::read_to_string("./main.abf").expect("Something went wrong reading the file")
    };
    let x = tokenize(contents.to_string());
    parse(x);
}
