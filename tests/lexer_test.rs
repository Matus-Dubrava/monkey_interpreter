#[cfg(test)]
mod tests {
    use monkey_interpreter::{lexer::lexer::Lexer, token::TokenType};

    fn assert_tokens_eq(exp_tokens: &Vec<TokenType>, lex: &mut Lexer) {
        for exp_tok in exp_tokens {
            let tok = lex.next_token();
            assert_eq!(*exp_tok, tok.r#type);
        }
    }

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
            Lexer::new(input.to_string()).expect("expected to be able to instantiate lexer");

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

        assert_tokens_eq(&exp_tokens, &mut lex);
    }

    #[test]
    fn should_tokenize_input_2() {
        let input = "
        !=/*5;
        5 < 10 > 5;
        ";

        let mut lex = Lexer::new(input.to_string())
            .expect("should be able to instantiate lexer with non-empty input");

        let exp_tokens = Vec::from([
            TokenType::BANG,
            TokenType::ASSIGN,
            TokenType::SLASH,
            TokenType::ASTERISK,
            TokenType::INT,
            TokenType::SEMICOLON,
            TokenType::INT,
            TokenType::LT,
            TokenType::INT,
            TokenType::GT,
            TokenType::INT,
            TokenType::SEMICOLON,
            TokenType::EOF,
        ]);

        assert_tokens_eq(&exp_tokens, &mut lex);
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
