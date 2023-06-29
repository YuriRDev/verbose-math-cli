use crate::token::{Token, TokenType};

pub enum Expression {
    BINARY(Box<Expression>, TokenType, Box<Expression>),
    UNARY(TokenType, Box<Expression>),
    LITERAL(i32),
    NONE
}

impl Expression {
    pub fn print(&self) {
        match self {
            Self::BINARY(left, token, right) => {
                print!("(");
                print!("{}", token.get_name());
                left.print();
                right.print();
                print!(") ");
            }
            Self::UNARY(token, right) => {
                print!(" (");
                print!("{}", token.get_name());
                right.print();
                print!(") ");
            }
            Self::LITERAL(value) => {
                print!(" (");
                print!("{}", value);
                print!(") ");
            },
            Self::NONE => {

            }
        }
    }
}
