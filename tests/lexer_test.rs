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

        let mut next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::LET);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::IDENT);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::ASSIGN);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::INT);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::SEMICOLON);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::LET);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::IDENT);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::ASSIGN);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::INT);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::SEMICOLON);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::LET);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::IDENT);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::ASSIGN);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::FUNCTION);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::LPAREN);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::IDENT);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::COMMA);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::IDENT);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::RPAREN);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::LBRACE);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::IDENT);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::PLUS);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::IDENT);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::RBRACE);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::SEMICOLON);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::LET);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::IDENT);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::ASSIGN);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::IDENT);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::LPAREN);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::IDENT);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::COMMA);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::IDENT);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::RPAREN);

        next_token = lex.next_token();
        assert_eq!(next_token.r#type, TokenType::SEMICOLON);
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
