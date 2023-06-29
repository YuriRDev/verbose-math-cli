mod parser;
mod scanner;
mod token;
use parser::Parser;

use crate::scanner::Scanner;

use std::{
    env::{self, Args},
    io::{self, BufRead},
};

fn main() {
    collect_args(env::args());
}

fn collect_args(args: Args) {
    let args: Vec<String> = args.collect();
    if args.len() == 2 {
        let mut scanner = Scanner::new(&args[1]);
        let parser = Parser::new(scanner.scan()).parse();
        println!("{}", parser);
    } else {
        println!("Type your query");
        let mut line = String::new();
        let stdin = io::stdin();
        loop {
            stdin.lock().read_line(&mut line).unwrap();
            let mut scanner = Scanner::new(&line);
            let parser = Parser::new(scanner.scan()).parse();
            println!("{}", parser);
        }
    }
}
