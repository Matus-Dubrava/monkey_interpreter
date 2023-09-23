mod helpers;

#[cfg(test)]
mod parsers_tests {
    use monkey_interpreter::ast::{
        ExpressionStatement, Identifier, InfixExpression, IntegerLiteral, LetStatement, Node,
        PrefixExpression, Program, Statement,
    };
    use monkey_interpreter::lexer::Lexer;
    use monkey_interpreter::parser::Parser;
    use monkey_interpreter::token::{Token, TokenType};

    use crate::helpers::*;

    #[test]
    fn test_operator_precedence_parsing() {
        struct OperatorPrecedenenceTest {
            input: String,
            expected: String,
        }

        impl OperatorPrecedenenceTest {
            fn new(input: &str, expected: &str) -> Self {
                OperatorPrecedenenceTest {
                    input: input.to_string(),
                    expected: expected.to_string(),
                }
            }
        }

        let test_cases = Vec::from([
            OperatorPrecedenenceTest::new("-a * b", "((-a) * b)"),
            OperatorPrecedenenceTest::new("!-a", "(!(-a))"),
            OperatorPrecedenenceTest::new("a + b + c", "((a + b) + c)"),
            OperatorPrecedenenceTest::new("a + b - c", "((a + b) - c)"),
            OperatorPrecedenenceTest::new("a * b * c", "((a * b) * c)"),
            OperatorPrecedenenceTest::new("a * b / c", "((a * b) / c)"),
            OperatorPrecedenenceTest::new("a + b / c", "(a + (b / c))"),
            OperatorPrecedenenceTest::new(
                "a + b * c + d / e - f",
                "(((a + (b * c)) + (d / e)) - f)",
            ),
            OperatorPrecedenenceTest::new("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
            OperatorPrecedenenceTest::new("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            OperatorPrecedenenceTest::new("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
            OperatorPrecedenenceTest::new(
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
        ]);

        for test_case in test_cases {
            let lex = Lexer::new(&test_case.input);
            let mut parser = Parser::new(lex);
            let program = parser.parse_program().unwrap();
            check_parse_errors(&parser);

            let program_string = program.to_string();
            assert_eq!(
                program_string, test_case.expected,
                "expected parsed program string to be {}, got={}",
                test_case.expected, program_string
            );
        }
    }

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

            validate_integer_literal(&prefix_expr.right, test_case.int_value);
        }
    }

    #[test]
    fn test_parsing_integer_infix_expressions() {
        struct InfixTest {
            input: String,
            left_value: i64,
            operator: String,
            right_value: i64,
        }

        impl InfixTest {
            fn new(input: &str, left_value: i64, operator: &str, right_value: i64) -> Self {
                InfixTest {
                    input: input.to_string(),
                    left_value,
                    operator: operator.to_string(),
                    right_value,
                }
            }
        }

        let test_cases = Vec::from([
            InfixTest::new("5 + 5", 5, "+", 5),
            InfixTest::new("5 - 5", 5, "-", 5),
            InfixTest::new("5 * 5", 5, "*", 5),
            InfixTest::new("5 / 5", 5, "/", 5),
            InfixTest::new("5 > 5", 5, ">", 5),
            InfixTest::new("5 < 5", 5, "<", 5),
            InfixTest::new("5 == 5", 5, "==", 5),
            InfixTest::new("5 != 5", 5, "!=", 5),
        ]);

        for test_case in test_cases {
            let lex = Lexer::new(&test_case.input);
            let mut parser = Parser::new(lex);
            let program = parser.parse_program().unwrap();
            check_parse_errors(&parser);

            dbg!(program.statements[0].to_string());

            assert_eq!(
                program.statements.len(),
                1,
                "expected {} statements, got={}",
                1,
                program.statements.len()
            );

            let expr_stmt = program.statements[0]
                .as_any()
                .downcast_ref::<ExpressionStatement>();

            assert!(
                expr_stmt.is_some(),
                "expected statement to be ExpressionStatement"
            );

            let infix_expr = expr_stmt
                .unwrap()
                .expression
                .as_any()
                .downcast_ref::<InfixExpression>();

            assert!(
                infix_expr.is_some(),
                "expected expression to be InfixExpression"
            );

            let infix_expr = infix_expr.unwrap();

            validate_integer_literal(&infix_expr.left, test_case.left_value);

            assert_eq!(
                infix_expr.operator, test_case.operator,
                "expected operator `{}`, got=`{}`",
                test_case.operator, infix_expr.operator
            );

            validate_integer_literal(&infix_expr.right, test_case.right_value);
        }
    }
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
            validate_let_statement(stmt, name);
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
            validate_return_statement(&stmt);
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
}
