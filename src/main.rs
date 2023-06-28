mod scanner;
mod token;
use crate::scanner::Scanner;

use std::{env::{self, Args}, process};

fn main() {
    // let mut a = Scanner::new("1 + 4 * 10 / 4");
    collect_args(env::args());

    let mut a = Scanner::new("5 plus 821231 pow 2 plus 4 plus 9");
    a.scan();
    a.print_tokens();
    
}

fn collect_args(args: Args) {
    let args: Vec<String> = args.collect();
    if args.len() < 2 {
        println!("Usage: cargo run \"5 plus 8 multiply 3\" ");
        process::exit(0);
    } else {
        let mut scanner = Scanner::new(&args[1]);
        scanner.scan();
        scanner.print_tokens();
    }
}
