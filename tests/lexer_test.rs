#[cfg(test)]
mod tests {
    use monkey_interpreter::{lexer::lexer::Lexer, token::TokenType};

    #[test]
    fn should_tokenize_input() {
        let input = "let five = 5;
        let ten = 10;
    
        let add = fn(x, y) {
            x + y
        };
    
        let result = add(five, ten);
        ";

        let mut lex =
            Lexer::new(String::from(input)).expect("expected to be able to instantiate lexer");

        let exp_tokens = Vec::from([
            TokenType::LET,
            TokenType::IDENT,
            TokenType::ASSIGN,
            TokenType::INT,
            TokenType::SEMICOLON,
            TokenType::LET,
            TokenType::IDENT,
            TokenType::ASSIGN,
            TokenType::INT,
            TokenType::SEMICOLON,
            TokenType::LET,
            TokenType::IDENT,
            TokenType::ASSIGN,
            TokenType::FUNCTION,
            TokenType::LPAREN,
            TokenType::IDENT,
            TokenType::COMMA,
            TokenType::IDENT,
            TokenType::RPAREN,
            TokenType::LBRACE,
            TokenType::IDENT,
            TokenType::PLUS,
            TokenType::IDENT,
            TokenType::RBRACE,
            TokenType::SEMICOLON,
            TokenType::LET,
            TokenType::IDENT,
            TokenType::ASSIGN,
            TokenType::IDENT,
            TokenType::LPAREN,
            TokenType::IDENT,
            TokenType::COMMA,
            TokenType::IDENT,
            TokenType::RPAREN,
            TokenType::SEMICOLON,
            TokenType::EOF,
        ]);

        for exp_tok in exp_tokens {
            let tok = lex.next_token();
            assert_eq!(exp_tok, tok.r#type);
        }
    }

    #[test]
    fn should_fail_if_instantiated_with_empty_string() {
        let input = "";

        let lex = Lexer::new(String::from(input));

        assert!(
            lex.is_err(),
            "should fail if instantiated with an empty string"
        );
    }
}
