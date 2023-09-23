#[cfg(test)]
mod parser_tests {
    use monkey_interpreter::{lexer::Lexer, token::TokenType};

    fn assert_tokens_eq(exp_tokens: &Vec<TokenType>, lex: &mut Lexer, debug: bool) {
        for exp_tok in exp_tokens {
            if debug {
                dbg!(&lex);
            }

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

        let mut lex = Lexer::new(&input.to_string());

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

        assert_tokens_eq(&exp_tokens, &mut lex, false);
    }

    #[test]
    fn should_tokenize_input_2() {
        let input = "
        ! =/*5;
        5 < 10 > 5;
        ";

        let mut lex = Lexer::new(&input.to_string());

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

        assert_tokens_eq(&exp_tokens, &mut lex, false);
    }

    #[test]
    fn should_tokenize_input_3() {
        let input = "
        if (5 < 10) {
            return true;
        } else {
            return false;
        }
        ";

        let mut lex = Lexer::new(&input.to_string());

        let exp_tokens = Vec::from([
            TokenType::IF,
            TokenType::LPAREN,
            TokenType::INT,
            TokenType::LT,
            TokenType::INT,
            TokenType::RPAREN,
            TokenType::LBRACE,
            TokenType::RETURN,
            TokenType::TRUE,
            TokenType::SEMICOLON,
            TokenType::RBRACE,
            TokenType::ELSE,
            TokenType::LBRACE,
            TokenType::RETURN,
            TokenType::FALSE,
            TokenType::SEMICOLON,
            TokenType::RBRACE,
            TokenType::EOF,
        ]);

        assert_tokens_eq(&exp_tokens, &mut lex, true);
    }

    #[test]
    fn should_tokenize_input_4() {
        let input = "
        10 == 10;
        10 != 9;
        ";

        let mut lex = Lexer::new(&input.to_string());

        let exp_tokens = Vec::from([
            TokenType::INT,
            TokenType::EQ,
            TokenType::INT,
            TokenType::SEMICOLON,
            TokenType::INT,
            TokenType::NOTEQ,
            TokenType::INT,
            TokenType::SEMICOLON,
        ]);

        assert_tokens_eq(&exp_tokens, &mut lex, false);
    }

    #[test]
    fn should_tokenize_input_5() {
        let input = "
        let x = 5.55;
        6.891 != 891.129;
        41.;
        1 + 2;
        ";

        let mut lex = Lexer::new(&input.to_string());

        let exp_tokens = Vec::from([
            TokenType::LET,
            TokenType::IDENT,
            TokenType::ASSIGN,
            TokenType::FLOAT,
            TokenType::SEMICOLON,
            TokenType::FLOAT,
            TokenType::NOTEQ,
            TokenType::FLOAT,
            TokenType::SEMICOLON,
            TokenType::ILLEGAL,
            TokenType::SEMICOLON,
            TokenType::INT,
            TokenType::PLUS,
            TokenType::INT,
            TokenType::SEMICOLON,
            TokenType::EOF,
        ]);

        assert_tokens_eq(&exp_tokens, &mut lex, false);
    }
}
