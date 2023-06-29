#[derive(Debug)]
pub enum TokenType {
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    POW,
    NUMBER,
}

pub struct Token {
    pub token_type: TokenType,
    pub value: i32,
}

impl Token {
    pub fn new(token_type: TokenType, value: i32) -> Token {
        Token { token_type, value }
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }
}

impl TokenType {
    pub fn get_name(&self) -> String {
        match self {
            Self::PLUS => "+".to_string(),
            Self::MINUS => "-".to_string(),
            Self::MULTIPLY => "*".to_string(),
            Self::DIVIDE => "/".to_string(),
            Self::POW => "^".to_string(),
            Self::NUMBER => "N".to_string(),
        }
    }
}
