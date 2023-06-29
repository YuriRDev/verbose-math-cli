mod scanner;
mod token;
mod parser;
mod expression;
use expression::Expression;
use parser::Parser;

use crate::scanner::Scanner;

use std::{env::{self, Args}, process};

fn main() {
    collect_args(env::args());
}

fn collect_args(args: Args) {
    let args: Vec<String> = args.collect();
    if args.len() < 2 {
        println!("Usage: cargo run \"5 plus 8 multiply 3\" ");
        process::exit(0);
    } else {
        let mut scanner = Scanner::new(&args[1]);
        let mut parser = Parser::new(scanner.scan()).parse();
        println!("{}", parser);
    }
}
