use crate::token::{Token, TokenType};
use std::process;

pub struct Scanner<'s> {
    source: &'s str,
    tokens: Vec<Token>,
    current: usize,
    line: usize,
}

// Constructor
impl<'s> Scanner<'s> {
    pub fn new<'a>(source: &'a str) -> Scanner<'a> {
        Scanner {
            source,
            tokens: Vec::new(),
            current: 0,
            line: 1,
        }
    }

    pub fn scan(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.scan_token();
        }

        &self.tokens
    }
}

impl<'s> Scanner<'s> {
    fn scan_token(&mut self) {
        let character = self.advance();

        match character {
            'p' => {
                if self.peek_word("plus") {
                    self.tokens.push(Token::new(TokenType::PLUS, 0));
                } else if self.peek_word("pow") {
                    self.tokens.push(Token::new(TokenType::POW, 0));
                } else {
                    self.throw_error("Unexpected character");
                }
            }
            'm' => {
                // Multiply, Minus
                if self.peek_word("multiply") {
                    self.tokens.push(Token::new(TokenType::MULTIPLY, 0));
                } else if self.peek_word("minus") {
                    self.tokens.push(Token::new(TokenType::MINUS, 0));
                } else {
                    self.throw_error("Unexpected character");
                }
            }
            'd' => {
                // Divide
                if self.peek_word("divide") {
                    self.tokens.push(Token::new(TokenType::DIVIDE, 0));
                } else {
                    self.throw_error("Unexpected character");
                }
            }
            ' ' => {}
            '\n' | '\r' | '\t' => {
                self.line += 1;
            }
            _ => {
                // Continuar lendo se o proximo caracter for um numero
                if !character.is_numeric() {
                    self.throw_error("Unexpected character");
                }

                let mut num_value_str = String::from(character);
                while self.peek().unwrap_or('a').is_numeric() {
                    match self.peek() {
                        Some(e) => {
                            if e.is_numeric() {
                                num_value_str.push(e);
                                self.advance();
                            }
                        }
                        _ => {}
                    };
                }

                self.tokens.push(Token::new(
                    TokenType::NUMBER,
                    num_value_str.trim().parse().unwrap(),
                ));
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let value = self.source.as_bytes()[self.current] as char;
        self.current += 1;
        value
    }

    fn peek_word(&mut self, word: &str) -> bool {
        let mut current_tmp = self.current;
        for i in 0..word.len() {
            let src_char = self.source.as_bytes()[current_tmp - 1] as char;
            let word_char = word.as_bytes()[i] as char;

            if src_char != word_char {
                return false;
            }

            if current_tmp > self.source.len() {
                return false;
            }

            current_tmp += 1;
        }
        self.current = current_tmp - 1;
        return true;
    }

    fn peek(&mut self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            Some(self.source.as_bytes()[self.current] as char)
        }
    }
}

/// Temporary error sender
impl<'s> Scanner<'s> {
    fn throw_error(&self, text: &str) {
        println!("[Error] {} at {}:{}", text, self.line, self.current);
        process::exit(0);
    }
}
