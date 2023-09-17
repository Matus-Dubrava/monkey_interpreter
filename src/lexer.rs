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

        pub fn make_two_char_token(
            &mut self,
            expected_next_ch: char,
            associated_tokey_type: TokenType,
        ) -> Option<Token> {
            let next_ch = self.peek_ahead();
            let tok: Token;

            if next_ch == expected_next_ch {
                tok = Token::from_str(
                    associated_tokey_type,
                    self.ch.to_string() + &next_ch.to_string(),
                );
                self.read_char();

                Some(tok)
            } else {
                None
            }
        }

        pub fn peek_ahead(&self) -> char {
            return self.input.chars().nth(self.read_position).unwrap_or('\0');
        }

        pub fn next_token(&mut self) -> Token {
            let tok: Token;

            self.skip_whitespace();

            match self.ch {
                '=' => {
                    tok = self
                        .make_two_char_token('=', TokenType::EQ)
                        .unwrap_or(Token::from_char(TokenType::ASSIGN, self.ch))
                }
                '!' => {
                    tok = self
                        .make_two_char_token('=', TokenType::NOTEQ)
                        .unwrap_or(Token::from_char(TokenType::BANG, self.ch))
                }
                ';' => tok = Token::from_char(TokenType::SEMICOLON, self.ch),
                '(' => tok = Token::from_char(TokenType::LPAREN, self.ch),
                ')' => tok = Token::from_char(TokenType::RPAREN, self.ch),
                ',' => tok = Token::from_char(TokenType::COMMA, self.ch),
                '+' => tok = Token::from_char(TokenType::PLUS, self.ch),
                '{' => tok = Token::from_char(TokenType::LBRACE, self.ch),
                '}' => tok = Token::from_char(TokenType::RBRACE, self.ch),
                '<' => tok = Token::from_char(TokenType::LT, self.ch),
                '>' => tok = Token::from_char(TokenType::GT, self.ch),
                '*' => tok = Token::from_char(TokenType::ASTERISK, self.ch),
                '/' => tok = Token::from_char(TokenType::SLASH, self.ch),
                '-' => tok = Token::from_char(TokenType::MINUS, self.ch),
                '\0' => tok = Token::from_char(TokenType::EOF, self.ch),
                _ => {
                    if self.ch.is_alphabetic() {
                        let literal = self.read_identifier();
                        // we are overshooting by one because there is a default read_char call
                        // at the end of this function and "read_integer" also reads one character ahead
                        // that is why we need to move the read position back one character
                        self.move_read_position_one_char_behind();

                        if let Some(keyword) = TokenType::get_keyword(&literal) {
                            tok = Token::from_str(keyword, literal);
                        } else {
                            tok = Token::from_str(TokenType::IDENT, literal);
                        }
                    } else if self.ch.is_numeric() {
                        let int_literal = self.read_integer();
                        // we are overshooting by one because there is a default read_char call
                        // at the end of this function and "read_integer" also reads one character ahead
                        // that is why we need to move the read position back one character
                        self.move_read_position_one_char_behind();

                        tok = Token::from_str(TokenType::INT, int_literal);
                    } else {
                        tok = Token::from_char(TokenType::ILLEGAL, self.ch);
                    }
                }
            }

            self.read_char();
            return tok;
        }
    }
}
