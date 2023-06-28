mod scanner;
mod token;

use crate::scanner::Scanner;

fn main() {
    // let mut a = Scanner::new("1 + 4 * 10 / 4");
    let mut a = Scanner::new("5 plus 821231 pow 2 plus 4 plus 9");
    a.scan();
    a.print_tokens();
}
