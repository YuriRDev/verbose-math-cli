use crate::token::{Token, TokenType};

pub struct Parser<'s> {
    tokens: &'s Vec<Token>,
}

impl<'s> Parser<'s> {
    pub fn new<'a>(tokens: &'a Vec<Token>) -> Parser<'a> {
        Parser { tokens }
    }

    pub fn parse(&mut self) -> i32 {
        self.scan(0, self.tokens.len())
    }

    fn scan(&mut self, esq: usize, dir: usize) -> i32 {
        if esq == dir {
            return 0;
        }
        let token = self.get_next_token(esq, dir);
        if token.is_none() {
            return 0;
        }

        let token_unwrap = &self.tokens[token.unwrap()];
        let left = self.scan(esq, token.unwrap());
        let right = self.scan(token.unwrap() + 1, dir);

        match token_unwrap.token_type {
            TokenType::NUMBER => {
                return token_unwrap.get_value();
            }
            TokenType::MINUS => return left - right,
            TokenType::PLUS => return left + right,
            TokenType::DIVIDE => return left / right,
            TokenType::MULTIPLY => return left * right,
            _ => return 0,
        }
    }

    fn get_next_token(&self, esq: usize, dir: usize) -> Option<usize> {
        // Get least precedende operator
        let next_plus = self.get_plus(esq, dir);
        if next_plus.is_some() {
            return Some(next_plus.unwrap());
        };

        let next_minus = self.get_minus(esq, dir);
        if next_minus.is_some() {
            return Some(next_minus.unwrap());
        };

        let next_mult = self.get_mult(esq, dir);
        if next_mult.is_some() {
            return Some(next_mult.unwrap());
        };

        let next_literal = self.get_literal(esq, dir);
        if next_literal.is_some() {
            return Some(next_literal.unwrap());
        }

        return None;
    }

    fn get_plus(&self, esq: usize, dir: usize) -> Option<usize> {
        for cnt in esq..dir {
            let current_token = &self.tokens[cnt].token_type;
            match current_token {
                TokenType::PLUS => {
                    return Some(cnt);
                }
                _ => {}
            }
        }
        None
    }

    fn get_minus(&self, esq: usize, dir: usize) -> Option<usize> {
        for cnt in esq..dir {
            let current_token = &self.tokens[cnt].token_type;
            match current_token {
                TokenType::MINUS => {
                    return Some(cnt);
                }
                _ => {}
            }
        }
        None
    }

    fn get_mult(&self, esq: usize, dir: usize) -> Option<usize> {
        for cnt in esq..dir {
            let current_token = &self.tokens[cnt].token_type;
            match current_token {
                TokenType::MULTIPLY => {
                    return Some(cnt);
                }
                TokenType::DIVIDE => {
                    return Some(cnt);
                }
                _ => {}
            }
        }
        None
    }

    fn get_literal(&self, esq: usize, dir: usize) -> Option<usize> {
        for cnt in esq..dir {
            let current_token = &self.tokens[cnt].token_type;
            match current_token {
                TokenType::NUMBER => {
                    return Some(cnt);
                }
                _ => {}
            }
        }
        None
    }
}
