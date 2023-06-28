#[derive(Debug)]
pub enum TokenType {
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    POW,
    NUMBER
}

pub struct Token {
    token_type: TokenType,
    value: i32,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, value: i32, line: usize) -> Token {
        Token {
            token_type,
            value,
            line,
        }
    }

    pub fn print(&self){
        println!("[line: {}] {:?} = {}", self.line, self.token_type, self.value);

    }
}