use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
    BANG,
    MINUS,
    ASTERISK,
    SLASH,
    LT,
    GT,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

impl TokenType {
    pub fn get_keyword(keyword: &String) -> Option<Self> {
        match keyword.as_str() {
            "let" => Some(Self::LET),
            "fn" => Some(Self::FUNCTION),
            "return" => Some(Self::RETURN),
            "true" => Some(Self::TRUE),
            "false" => Some(Self::FALSE),
            "if" => Some(Self::IF),
            "else" => Some(Self::ELSE),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub r#type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, ch: char) -> Self {
        Token {
            r#type: token_type,
            literal: String::from(ch),
        }
    }
}
