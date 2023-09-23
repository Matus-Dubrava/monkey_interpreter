mod helpers;

#[cfg(test)]
mod parsers_tests {
    use std::any::Any;

    use monkey_interpreter::ast::{Identifier, LetStatement, Node, Program, Statement};
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
            OperatorPrecedenenceTest::new("true", "true"),
            OperatorPrecedenenceTest::new("false", "false"),
            OperatorPrecedenenceTest::new("3 > 5 == false", "((3 > 5) == false)"),
            OperatorPrecedenenceTest::new("3 < 5 == true", "((3 < 5) == true)"),
        ]);

        for test_case in test_cases {
            let mut parser = Parser::from_str(test_case.input.as_str());
            let program = parser.parse_program();
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
    fn test_parsing_prefix_expression() {
        struct PrefixTest {
            input: String,
            operator: String,
            right: Box<dyn Any>,
        }

        impl PrefixTest {
            fn new(input: &str, operator: &str, right: Box<dyn Any>) -> Self {
                PrefixTest {
                    input: input.to_string(),
                    operator: operator.to_string(),
                    right,
                }
            }
        }

        let mut test_cases: Vec<PrefixTest> = Vec::new();

        let right: Box<dyn Any> = Box::new(5);
        test_cases.push(PrefixTest::new("!5", "!", right));

        let right: Box<dyn Any> = Box::new(5);
        test_cases.push(PrefixTest::new("-5", "-", right));

        let right: Box<dyn Any> = Box::new(123.51);
        test_cases.push(PrefixTest::new("-123.51", "-", right));

        let right: Box<dyn Any> = Box::new("my_var");
        test_cases.push(PrefixTest::new("!my_var", "!", right));

        for test_case in test_cases {
            let mut parser = Parser::from_str(test_case.input.as_str());
            let program = parser.parse_program();
            validate_program_length(&program, 1);

            let expr = get_and_assert_expression(&program.statements[0]);
            validate_prefix_expression(expr, test_case.operator, &test_case.right);

            assert_eq!(
                program.to_string(),
                format!("({})", &test_case.input),
                "expected program to be `{}`, got=`{}`",
                format!("({})", &test_case.input),
                program.to_string()
            );
        }
    }

    #[test]
    fn test_parsing_infix_expressions() {
        struct InfixTest {
            input: String,
            left: Box<dyn Any>,
            operator: String,
            right: Box<dyn Any>,
        }

        impl InfixTest {
            fn new(input: &str, left: Box<dyn Any>, operator: &str, right: Box<dyn Any>) -> Self {
                InfixTest {
                    input: input.to_string(),
                    left,
                    operator: operator.to_string(),
                    right,
                }
            }
        }

        let mut test_cases: Vec<InfixTest> = Vec::new();

        let left: Box<dyn Any> = Box::new(5);
        let right: Box<dyn Any> = Box::new(5);
        test_cases.push(InfixTest::new("5 + 5", left, "+", right));

        let left: Box<dyn Any> = Box::new(5.12);
        let right: Box<dyn Any> = Box::new(25);
        test_cases.push(InfixTest::new("5.12 - 25", left, "-", right));

        let left: Box<dyn Any> = Box::new("a");
        let right: Box<dyn Any> = Box::new(12);
        test_cases.push(InfixTest::new("a * 12", left, "*", right));

        let left: Box<dyn Any> = Box::new("a");
        let right: Box<dyn Any> = Box::new("my_var");
        test_cases.push(InfixTest::new("a / my_var", left, "/", right));

        let left: Box<dyn Any> = Box::new("a");
        let right: Box<dyn Any> = Box::new(0.1);
        test_cases.push(InfixTest::new("a > 0.1", left, ">", right));

        let left: Box<dyn Any> = Box::new(true);
        let right: Box<dyn Any> = Box::new(false);
        test_cases.push(InfixTest::new("true < false", left, "<", right));

        let left: Box<dyn Any> = Box::new(5.55);
        let right: Box<dyn Any> = Box::new(false);
        test_cases.push(InfixTest::new("5.55 == false", left, "==", right));

        let left: Box<dyn Any> = Box::new("AnotherVar");
        let right: Box<dyn Any> = Box::new("my_var");
        test_cases.push(InfixTest::new("AnotherVar != my_var", left, "!=", right));

        for test_case in test_cases {
            let mut parser = Parser::from_str(test_case.input.as_str());
            let program = parser.parse_program();

            check_parse_errors(&parser);
            validate_program_length(&program, 1);

            let expr = get_and_assert_expression(&program.statements[0]);

            validate_infix_expression(expr, &test_case.left, test_case.operator, &test_case.right);

            assert_eq!(
                program.to_string(),
                format!("({})", test_case.input),
                "expected program to be `{}`, got=`{}`",
                format!("({})", test_case.input),
                program.to_string()
            )
        }
    }
    #[test]
    fn should_parse_let_statements() {
        let input = "
        let x = 5;
        let y = 10;
        let foobar = 12345;
        ";

        let mut parser = Parser::from_str(input);
        let program = parser.parse_program();

        check_parse_errors(&parser);
        validate_program_length(&program, 3);

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

        let mut parser = Parser::from_str(input);
        let program = parser.parse_program();

        validate_program_length(&program, 3);
        check_parse_errors(&parser);

        for stmt in program.statements {
            validate_return_statement(&stmt);
        }
    }

    #[test]
    fn should_parse_boolean_literals() {
        let input = "
        true;
        false;
        ";

        let expected_values = Vec::from([true, false]);
        let mut parser = Parser::from_str(input);
        let program = parser.parse_program();

        validate_program_length(&program, 2);
        check_parse_errors(&parser);

        for (stmt, expected_value) in program.statements.iter().zip(expected_values.iter()) {
            let expr = get_and_assert_expression(&stmt);
            validate_boolean_literal(expr, expected_value);
        }
    }

    #[test]
    fn should_parse_identifier_expression() {
        let input = "foobar;";
        let mut parser = Parser::from_str(input);
        let program = parser.parse_program();

        check_parse_errors(&parser);
        validate_program_length(&program, 1);

        let expr = get_and_assert_expression(&program.statements[0]);
        validate_identifier(&expr, "foobar");
        assert_eq!(program.to_string(), "foobar");
    }

    #[test]
    fn should_parse_integer_literal_expression() {
        let input = "5;";
        let mut parser = Parser::from_str(input);
        let program = parser.parse_program();

        check_parse_errors(&parser);
        validate_program_length(&program, 1);

        let expr = get_and_assert_expression(&program.statements[0]);
        validate_integer_literal(&expr, 5);
        assert_eq!(program.to_string(), "5");
    }

    #[test]
    fn should_record_parsing_errors() {
        let input = "
        let x 5;
        let = 10;
        let 12345;
        ";

        let mut parser = Parser::from_str(input);
        parser.parse_program();

        assert!(
            parser.get_errors().len() >= 3,
            "expected at least 3 errors, got={}",
            parser.get_errors().len()
        );
    }

    #[test]
    fn test_to_string_method_manual_let_statement() {
        let identifier = Identifier {
            token: Token::from_str(TokenType::IDENT, "my_var"),
            value: "my_var".to_string(),
        };

        let expression = Box::new(Identifier {
            token: Token::from_str(TokenType::IDENT, "another_var"),
            value: "another_var".to_string(),
        });

        let let_statement = Box::new(LetStatement::new(
            Token::from_str(TokenType::LET, "let"),
            identifier,
            expression,
        ));

        let mut statements: Vec<Box<dyn Statement>> = Vec::new();
        statements.push(let_statement);

        let program = Program::from_statements(statements);
        assert_eq!(program.to_string(), "let my_var = another_var;");
    }
}
