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
}

impl FromStr for TokenType {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "let" => Ok(TokenType::LET),
            "fn" => Ok(TokenType::FUNCTION),
            _ => Err(color_eyre::eyre::eyre!(
                "failed to parse string into TokeType"
            )),
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
