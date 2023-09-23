#[cfg(test)]
mod parser_tests {
    use monkey_interpreter::{lexer::Lexer, token::Token, token::TokenType};

    fn assert_tokens_eq(expected_tokens: &Vec<Token>, tokens: &Vec<Token>) {
        assert_eq!(
            expected_tokens.len(),
            tokens.len(),
            "lenght of expected tokens does't match actual length, expected=`{}`, got=`{}`",
            expected_tokens.len(),
            tokens.len()
        );

        for (expected_token, token) in expected_tokens.iter().zip(tokens.iter()) {
            assert_eq!(expected_token, token);
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
            Token::from_str(TokenType::LET, "let"),
            Token::from_str(TokenType::IDENT, "five"),
            Token::from_str(TokenType::ASSIGN, "="),
            Token::from_str(TokenType::INT, "5"),
            Token::from_str(TokenType::SEMICOLON, ";"),
            Token::from_str(TokenType::LET, "let"),
            Token::from_str(TokenType::IDENT, "ten"),
            Token::from_str(TokenType::ASSIGN, "="),
            Token::from_str(TokenType::INT, "10"),
            Token::from_str(TokenType::SEMICOLON, ";"),
            Token::from_str(TokenType::LET, "let"),
            Token::from_str(TokenType::IDENT, "add"),
            Token::from_str(TokenType::ASSIGN, "="),
            Token::from_str(TokenType::FUNCTION, "fn"),
            Token::from_str(TokenType::LPAREN, "("),
            Token::from_str(TokenType::IDENT, "x"),
            Token::from_str(TokenType::COMMA, ","),
            Token::from_str(TokenType::IDENT, "y"),
            Token::from_str(TokenType::RPAREN, ")"),
            Token::from_str(TokenType::LBRACE, "{"),
            Token::from_str(TokenType::IDENT, "x"),
            Token::from_str(TokenType::PLUS, "+"),
            Token::from_str(TokenType::IDENT, "y"),
            Token::from_str(TokenType::RBRACE, "}"),
            Token::from_str(TokenType::SEMICOLON, ";"),
            Token::from_str(TokenType::LET, "let"),
            Token::from_str(TokenType::IDENT, "result"),
            Token::from_str(TokenType::ASSIGN, "="),
            Token::from_str(TokenType::IDENT, "add"),
            Token::from_str(TokenType::LPAREN, "("),
            Token::from_str(TokenType::IDENT, "five"),
            Token::from_str(TokenType::COMMA, ","),
            Token::from_str(TokenType::IDENT, "ten"),
            Token::from_str(TokenType::RPAREN, ")"),
            Token::from_str(TokenType::SEMICOLON, ";"),
            Token::from_str(TokenType::EOF, "\0"),
        ]);

        let tokens = lex.get_all_tokens();

        assert_tokens_eq(&exp_tokens, &tokens);
    }

    #[test]
    fn should_tokenize_input_2() {
        let input = "
        ! =/*5;
        5 < 10 > 5;
        ";

        let mut lex = Lexer::new(&input.to_string());

        let exp_tokens = Vec::from([
            Token::from_str(TokenType::BANG, "!"),
            Token::from_str(TokenType::ASSIGN, "="),
            Token::from_str(TokenType::SLASH, "/"),
            Token::from_str(TokenType::ASTERISK, "*"),
            Token::from_str(TokenType::INT, "5"),
            Token::from_str(TokenType::SEMICOLON, ";"),
            Token::from_str(TokenType::INT, "5"),
            Token::from_str(TokenType::LT, "<"),
            Token::from_str(TokenType::INT, "10"),
            Token::from_str(TokenType::GT, ">"),
            Token::from_str(TokenType::INT, "5"),
            Token::from_str(TokenType::SEMICOLON, ";"),
            Token::from_str(TokenType::EOF, "\0"),
        ]);

        let tokens = lex.get_all_tokens();
        assert_tokens_eq(&exp_tokens, &tokens);
    }

    #[test]
    fn should_tokenize_should_tokenize_if_else_statement() {
        let input = "
        if (5 < 10) {
            return true;
        } else {
            return false;
        }
        ";

        let mut lex = Lexer::new(&input.to_string());

        let exp_tokens = Vec::from([
            Token::from_str(TokenType::IF, "if"),
            Token::from_str(TokenType::LPAREN, "("),
            Token::from_str(TokenType::INT, "5"),
            Token::from_str(TokenType::LT, "<"),
            Token::from_str(TokenType::INT, "10"),
            Token::from_str(TokenType::RPAREN, ")"),
            Token::from_str(TokenType::LBRACE, "{"),
            Token::from_str(TokenType::RETURN, "return"),
            Token::from_str(TokenType::TRUE, "true"),
            Token::from_str(TokenType::SEMICOLON, ";"),
            Token::from_str(TokenType::RBRACE, "}"),
            Token::from_str(TokenType::ELSE, "else"),
            Token::from_str(TokenType::LBRACE, "{"),
            Token::from_str(TokenType::RETURN, "return"),
            Token::from_str(TokenType::FALSE, "false"),
            Token::from_str(TokenType::SEMICOLON, ";"),
            Token::from_str(TokenType::RBRACE, "}"),
            Token::from_str(TokenType::EOF, "\0"),
        ]);

        let tokens = lex.get_all_tokens();
        assert_tokens_eq(&exp_tokens, &tokens)
    }

    #[test]
    fn should_tokenize_equals_and_not_equals() {
        let input = "
        1001 == 1001;
        192 != 99;
        ";

        let mut lex = Lexer::new(&input.to_string());

        let exp_tokens = Vec::from([
            Token::from_str(TokenType::INT, "1001"),
            Token::from_str(TokenType::EQ, "=="),
            Token::from_str(TokenType::INT, "1001"),
            Token::from_str(TokenType::SEMICOLON, ";"),
            Token::from_str(TokenType::INT, "192"),
            Token::from_str(TokenType::NOTEQ, "!="),
            Token::from_str(TokenType::INT, "99"),
            Token::from_str(TokenType::SEMICOLON, ";"),
            Token::from_str(TokenType::EOF, "\0"),
        ]);

        let tokens = lex.get_all_tokens();
        assert_tokens_eq(&exp_tokens, &tokens);
    }

    #[test]
    fn should_tokenize_floats() {
        let input = "
        let some_x = 5.55;
        6.891 != 891.129;
        41.;
        ";

        let mut lex = Lexer::new(&input.to_string());

        let exp_tokens = Vec::from([
            Token::from_str(TokenType::LET, "let"),
            Token::from_str(TokenType::IDENT, "some_x"),
            Token::from_str(TokenType::ASSIGN, "="),
            Token::from_str(TokenType::FLOAT, "5.55"),
            Token::from_str(TokenType::SEMICOLON, ";"),
            Token::from_str(TokenType::FLOAT, "6.891"),
            Token::from_str(TokenType::NOTEQ, "!="),
            Token::from_str(TokenType::FLOAT, "891.129"),
            Token::from_str(TokenType::SEMICOLON, ";"),
            Token::from_str(TokenType::ILLEGAL, "illegal"),
            Token::from_str(TokenType::SEMICOLON, ";"),
            Token::from_str(TokenType::EOF, "\0"),
        ]);

        let tokens = lex.get_all_tokens();

        assert_tokens_eq(&exp_tokens, &tokens);
    }
}
