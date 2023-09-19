#[cfg(test)]
mod parsers_tests {
    use monkey_interpreter::ast::{
        Expression, ExpressionStatement, Identifier, IntegerLiteral, LetStatement, Node,
        PrefixExpression, Program, ReturnStatement, Statement,
    };
    use monkey_interpreter::lexer::Lexer;
    use monkey_interpreter::parser::Parser;
    use monkey_interpreter::token::{Token, TokenType};

    #[test]
    fn test_parsing_integer_prefix_expression() {
        struct PrefixTest {
            input: String,
            operator: String,
            int_value: i64,
        }

        impl PrefixTest {
            fn new(input: &str, operator: &str, int_value: i64) -> Self {
                PrefixTest {
                    input: input.to_string(),
                    operator: operator.to_string(),
                    int_value,
                }
            }
        }

        let test_cases = Vec::from([
            PrefixTest::new("!5", "!", 5),
            PrefixTest::new("-15", "-", 15),
        ]);

        for test_case in test_cases {
            let lex = Lexer::new(&test_case.input);
            let mut parser = Parser::new(lex);
            let program = parser.parse_program().unwrap();

            assert_eq!(
                program.statements.len(),
                1,
                "expected one statement to be parsed"
            );

            let expr_stmt = program.statements[0]
                .as_any()
                .downcast_ref::<ExpressionStatement>();

            assert_eq!(
                expr_stmt.is_some(),
                true,
                "expected to be able to downcast Statement to ExpressionStatement"
            );

            let prefix_expr = expr_stmt
                .unwrap()
                .expression
                .as_any()
                .downcast_ref::<PrefixExpression>();

            assert_eq!(
                prefix_expr.is_some(),
                true,
                "expression is not PrefixExpression"
            );

            let prefix_expr = prefix_expr.unwrap();
            assert_eq!(
                prefix_expr.operator, test_case.operator,
                "expected operator {}, got={}",
                test_case.operator, prefix_expr.operator
            );

            test_integer_literal(&prefix_expr.right, test_case.int_value);
        }
    }

    // #[test]
    // fn test_parsing_integer_infix_expressions() {
    //     struct InfixTest {
    //         input: String,
    //         left_value: i64,
    //         operator: String,
    //         right_value: i64,
    //     }

    //     impl InfixTest {
    //         fn new(input: &str, left_value: i64, operator: &str, right_value: i64) -> Self {
    //             InfixTest {
    //                 input: input.to_string(),
    //                 left_value,
    //                 operator: operator.to_string(),
    //                 right_value,
    //             }
    //         }
    //     }

    //     let stmts = Vec::from([
    //         InfixTest::new("5 + 5", 5, "+", 5),
    //         InfixTest::new("5 - 5", 5, "-", 5),
    //         InfixTest::new("5 * 5", 5, "*", 5),
    //         InfixTest::new("5 / 5", 5, "/", 5),
    //         InfixTest::new("5 > 5", 5, ">", 5),
    //         InfixTest::new("5 < 5", 5, "<", 5),
    //         InfixTest::new("5 == 5", 5, "==", 5),
    //         InfixTest::new("5 != 5", 5, "!=", 5),
    //     ]);

    //     for stmt in stmts {
    //         let lex = Lexer::new(&stmt.input);
    //         let mut parser = Parser::new(lex);
    //         let program = parser.parse_program().unwrap();
    //         check_parse_errors(&parser);

    //         assert_eq!(
    //             program.statements.len(),
    //             1,
    //             "expected {} statements, got={}",
    //             1,
    //             program.statements.len()
    //         );

    //         let expr_stmt = program.statements[0].as_any().downcast_ref::<ExpressionStatement>();
    //         assert_eq!(expr_stmt.is_some(), true, "expected statement to be ExpressionStatement");

    //         let expr_st
    //         // continue once InfixExpression is implemented
    //     }
    // }
    #[test]
    fn should_parse_let_statements() {
        let input = "
        let x = 5;
        let y = 10;
        let foobar = 12345;
        ";

        let lex = Lexer::new(&input.to_string());
        let mut parser = Parser::new(lex);

        let program = parser.parse_program();
        assert!(program.is_some());
        check_parse_errors(&parser);

        let program = program.unwrap();
        assert_eq!(program.statements.len(), 3);

        let tests = ["x".to_string(), "y".to_string(), "foobar".to_string()];

        for (i, name) in tests.iter().enumerate() {
            let stmt = &program.statements[i];
            test_let_statement(stmt, name);
        }
    }

    #[test]
    fn should_parse_return_statements() {
        let input = "
        return 1;
        return 10;
        return 10000;
        ";

        let lex = Lexer::new(&input.to_string());
        let mut parser = Parser::new(lex);

        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 3);

        check_parse_errors(&parser);

        for stmt in program.statements {
            test_return_statement(&stmt);
        }
    }

    #[test]
    fn should_parse_identifier_expression() {
        let input = "foobar;".to_string();

        let lex = Lexer::new(&input);
        let mut parser = Parser::new(lex);
        let program = parser.parse_program().unwrap();
        check_parse_errors(&parser);

        assert_eq!(
            program.statements.len(),
            1,
            "expected one statement to be parsed"
        );

        let expr_stmt = program.statements[0]
            .as_any()
            .downcast_ref::<ExpressionStatement>();

        assert_eq!(
            expr_stmt.is_some(),
            true,
            "expected to be able to downcast Statement to ExpressionStatement"
        );

        let ident = expr_stmt
            .unwrap()
            .expression
            .as_any()
            .downcast_ref::<Identifier>();

        assert_eq!(
            ident.is_some(),
            true,
            "expected to be able to downcast ExpressionStatement to Identifier"
        );

        let ident = ident.unwrap();
        assert_eq!(ident.value, "foobar");
        assert_eq!(ident.token_literal(), "foobar");
    }

    #[test]
    fn should_parse_integer_literal_expression() {
        let input = "5;".to_string();

        let lex = Lexer::new(&input);
        let mut parser = Parser::new(lex);
        let program = parser.parse_program().unwrap();

        assert_eq!(
            program.statements.len(),
            1,
            "expected one statement to be parsed"
        );

        let expr_stmt = program.statements[0]
            .as_any()
            .downcast_ref::<ExpressionStatement>();

        assert_eq!(
            expr_stmt.is_some(),
            true,
            "expected to be able to downcast Statement to ExpressionStatement"
        );

        let integer = expr_stmt
            .unwrap()
            .expression
            .as_any()
            .downcast_ref::<IntegerLiteral>();

        assert_eq!(
            integer.is_some(),
            true,
            "expected to be able to downcast Statement to IntegerLiteral"
        );

        let integer = integer.unwrap();
        assert_eq!(integer.value, 5);
        assert_eq!(integer.token_literal(), "5");
    }

    #[test]
    fn should_record_parsing_errors() {
        let input = "
        let x 5;
        let = 10;
        let 12345;
        ";

        let lex = Lexer::new(&input.to_string());
        let mut parser = Parser::new(lex);

        parser.parse_program().unwrap();
        assert!(
            parser.get_errors().len() >= 3,
            "expected at least 3 errors, got={}",
            parser.get_errors().len()
        );
    }

    #[test]
    fn test_to_string_method_manual_let_statement() {
        let identifier = Identifier {
            token: Token::from_str(TokenType::IDENT, "my_var".to_string()),
            value: "my_var".to_string(),
        };

        let expression = Box::new(Identifier {
            token: Token::from_str(TokenType::IDENT, "another_var".to_string()),
            value: "another_var".to_string(),
        });

        let let_statement = Box::new(LetStatement::new(
            Token::from_str(TokenType::LET, "let".to_string()),
            identifier,
            expression,
        ));

        let mut statements: Vec<Box<dyn Statement>> = Vec::new();
        statements.push(let_statement);

        let program = Program::from_statements(statements);
        assert_eq!(program.to_string(), "let my_var = another_var;");
    }

    fn check_parse_errors(parser: &Parser) {
        let errors = parser.get_errors();

        for err in errors {
            eprintln!("parser error: {}", err);
        }

        assert_eq!(
            errors.len(),
            0,
            "encoutered {} errors during parsing",
            errors.len()
        );
    }

    fn test_integer_literal(int_literal: &Box<dyn Expression>, value: i64) {
        let int = int_literal.as_any().downcast_ref::<IntegerLiteral>();
        assert_eq!(
            int.is_some(),
            true,
            "expected expression to be IntegerLiteral"
        );

        let int = int.unwrap();

        assert_eq!(
            int.value, value,
            "expected value to be {}, got={}",
            int.value, value
        );

        assert_eq!(
            int.token_literal(),
            value.to_string(),
            "expected token literal to be {}, got={}",
            int.token_literal(),
            value.to_string()
        );
    }

    fn test_return_statement(stmt: &Box<dyn Statement>) {
        assert_eq!(
            stmt.token_literal(),
            "return",
            "statement's token literal is not 'return', got={}",
            stmt.token_literal()
        );

        let return_stmt = stmt.as_any().downcast_ref::<ReturnStatement>();
        assert_eq!(
            return_stmt.is_none(),
            false,
            "expected statement to be ReturnStatement"
        );
    }

    fn test_let_statement(stmt: &Box<dyn Statement>, name: &String) {
        assert_eq!(
            stmt.token_literal(),
            "let",
            "expected token literal to be `let`, got={}",
            stmt.token_literal()
        );

        let let_stmt = stmt.as_any().downcast_ref::<LetStatement>();
        assert_eq!(
            let_stmt.is_some(),
            true,
            "expected statement to be LetStatement"
        );

        let let_stmt = let_stmt.unwrap();
        assert_eq!(
            &let_stmt.name.value, name,
            "LetStatement name is not {}, got={}",
            name, let_stmt.name.value
        );
        assert_eq!(
            &let_stmt.name.token_literal(),
            name,
            "statement name is not {}, got={}",
            name,
            let_stmt.name.token_literal()
        );
    }
}
