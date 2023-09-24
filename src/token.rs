#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    EQ,
    NOTEQ,
    FLOAT,
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

impl ToString for TokenType {
    fn to_string(&self) -> String {
        match self {
            TokenType::ILLEGAL => "ILLEGAL".to_string(),
            TokenType::EOF => "EOF".to_string(),
            TokenType::IDENT => "IDENT".to_string(),
            TokenType::INT => "INT".to_string(),
            TokenType::ASSIGN => "ASSIGN".to_string(),
            TokenType::PLUS => "PLUS".to_string(),
            TokenType::COMMA => "COMMA".to_string(),
            TokenType::SEMICOLON => "SEMICOLON".to_string(),
            TokenType::LPAREN => "LPAREN".to_string(),
            TokenType::RPAREN => "RPAREN".to_string(),
            TokenType::LBRACE => "LBRACE".to_string(),
            TokenType::RBRACE => "RBRACE".to_string(),
            TokenType::FUNCTION => "FUNCTION".to_string(),
            TokenType::LET => "LET".to_string(),
            TokenType::BANG => "BANG".to_string(),
            TokenType::MINUS => "MINUS".to_string(),
            TokenType::ASTERISK => "ASTERISK".to_string(),
            TokenType::SLASH => "SLASH".to_string(),
            TokenType::LT => "LT".to_string(),
            TokenType::GT => "GT".to_string(),
            TokenType::TRUE => "TRUE".to_string(),
            TokenType::FALSE => "FALSE".to_string(),
            TokenType::IF => "IF".to_string(),
            TokenType::ELSE => "ELSE".to_string(),
            TokenType::RETURN => "RETURN".to_string(),
            TokenType::EQ => "EQ".to_string(),
            TokenType::NOTEQ => "NOTEQ".to_string(),
            TokenType::FLOAT => "FLOAT".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub r#type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn from_char(token_type: TokenType, ch: char) -> Self {
        Token {
            r#type: token_type,
            literal: ch.to_string(),
        }
    }

    pub fn from_str(token_type: TokenType, s: &str) -> Self {
        Token {
            r#type: token_type,
            literal: s.to_string(),
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("[`{}`: `{}`]", self.r#type.to_string(), self.literal)
    }
}
