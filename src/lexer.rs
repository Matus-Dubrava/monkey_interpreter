pub mod lexer {
    use crate::token::{Token, TokenType};

    #[derive(Debug, Clone)]
    pub struct Lexer {
        input: String,
        position: usize,
        read_position: usize,
        ch: char,
    }

    impl Lexer {
        pub fn new(input: String) -> Result<Lexer, color_eyre::Report> {
            let ch = input.chars().nth(0);
            match ch {
                Some(ch) => Ok(Lexer {
                    input,
                    position: 0,
                    read_position: 1,
                    ch,
                }),
                None => Err(color_eyre::eyre::eyre!(
                    "cannot instantiate Lexer with an empty string"
                )),
            }
        }

        pub fn read_char(&mut self) {
            let next_char = self.input.chars().nth(self.read_position);
            match next_char {
                Some(ch) => self.ch = ch,
                None => self.ch = '\0',
            }
            self.position = self.read_position;
            self.read_position += 1;
        }

        pub fn move_read_position_one_char_behind(&mut self) {
            self.position -= 1;
            self.read_position -= 1;
            self.ch = self
                .input
                .chars()
                .nth(self.position)
                .expect("expected to be able to move one position behind");
        }

        pub fn skip_whitespace(&mut self) {
            while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
                self.read_char();
            }
        }

        pub fn read_integer(&mut self) -> String {
            let position = self.position;
            while self.ch.is_numeric() {
                self.read_char()
            }
            return String::from(&self.input[position..self.position]);
        }

        pub fn read_identifier(&mut self) -> String {
            let position = self.position;
            while self.ch.is_alphabetic() || self.ch == '_' {
                self.read_char()
            }
            return String::from(&self.input[position..self.position]);
        }

        pub fn next_token(&mut self) -> Token {
            let tok: Token;

            self.skip_whitespace();

            match self.ch {
                '=' => tok = Token::new(TokenType::ASSIGN, self.ch),
                ';' => tok = Token::new(TokenType::SEMICOLON, self.ch),
                '(' => tok = Token::new(TokenType::LPAREN, self.ch),
                ')' => tok = Token::new(TokenType::RPAREN, self.ch),
                ',' => tok = Token::new(TokenType::COMMA, self.ch),
                '+' => tok = Token::new(TokenType::PLUS, self.ch),
                '{' => tok = Token::new(TokenType::LBRACE, self.ch),
                '}' => tok = Token::new(TokenType::RBRACE, self.ch),
                '<' => tok = Token::new(TokenType::LT, self.ch),
                '>' => tok = Token::new(TokenType::GT, self.ch),
                '!' => tok = Token::new(TokenType::BANG, self.ch),
                '*' => tok = Token::new(TokenType::ASTERISK, self.ch),
                '/' => tok = Token::new(TokenType::SLASH, self.ch),
                '-' => tok = Token::new(TokenType::MINUS, self.ch),
                '\0' => tok = Token::new(TokenType::EOF, self.ch),
                _ => {
                    if self.ch.is_alphabetic() {
                        let literal = self.read_identifier();
                        // we are overshooting by one because there is a default read_char call
                        // at the end of this function and "read_integer" also reads one character ahead
                        // that is why we need to move the read position back one character
                        self.move_read_position_one_char_behind();

                        match literal.as_str() {
                            "let" => {
                                tok = Token {
                                    r#type: TokenType::LET,
                                    literal,
                                }
                            }
                            "fn" => {
                                tok = Token {
                                    r#type: TokenType::FUNCTION,
                                    literal,
                                }
                            }
                            "if" => {
                                tok = Token {
                                    r#type: TokenType::IF,
                                    literal,
                                }
                            }
                            "else" => {
                                tok = Token {
                                    r#type: TokenType::ELSE,
                                    literal,
                                }
                            }
                            "return" => {
                                tok = Token {
                                    r#type: TokenType::RETURN,
                                    literal,
                                }
                            }
                            "true" => {
                                tok = Token {
                                    r#type: TokenType::TRUE,
                                    literal,
                                }
                            }
                            "false" => {
                                tok = Token {
                                    r#type: TokenType::FALSE,
                                    literal,
                                }
                            }
                            _ => {
                                tok = Token {
                                    r#type: TokenType::IDENT,
                                    literal,
                                }
                            }
                        }
                    } else if self.ch.is_numeric() {
                        let literal = self.read_integer();
                        // we are overshooting by one because there is a default read_char call
                        // at the end of this function and "read_integer" also reads one character ahead
                        // that is why we need to move the read position back one character
                        self.move_read_position_one_char_behind();

                        tok = Token {
                            r#type: TokenType::INT,
                            literal,
                        }
                    } else {
                        tok = Token::new(TokenType::ILLEGAL, self.ch);
                    }
                }
            }

            self.read_char();
            return tok;
        }
    }
}
