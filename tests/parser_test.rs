mod helpers;

#[cfg(test)]
mod parsers_tests {
    use std::any::Any;

    use monkey_interpreter::ast::{Expression, Identifier, LetStatement, Node, Program, Statement};
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
            OperatorPrecedenenceTest::new("3 + 4; -5 * 5", "(3 + 4); ((-5) * 5)"),
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
            // grouped expressions
            OperatorPrecedenenceTest::new("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
            OperatorPrecedenenceTest::new("(5 + 5) * 2", "((5 + 5) * 2)"),
            OperatorPrecedenenceTest::new("2 / (5 + 5)", "(2 / (5 + 5))"),
            OperatorPrecedenenceTest::new("-(5 + 5)", "(-(5 + 5))"),
            OperatorPrecedenenceTest::new("!(true == true)", "(!(true == true))"),
            // call expressions
            OperatorPrecedenenceTest::new("a + add(b * c) + d", "((a + add((b * c))) + d)"),
            OperatorPrecedenenceTest::new(
                "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
                "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
            ),
            OperatorPrecedenenceTest::new(
                "add(a + b + c * d / f + g)",
                "add((((a + b) + ((c * d) / f)) + g))",
            ),
        ]);

        for test_case in test_cases {
            let mut parser = Parser::from_str(test_case.input.as_str());
            let program = parser.parse_program();
            check_parse_errors(&parser);

            assert_eq!(
                program.to_string().trim_end_matches(";"),
                test_case.expected,
                "expected parsed program string to be {}, got={}",
                test_case.expected,
                program.to_string().trim_end_matches(";")
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
                program.to_string().trim_end_matches(";"),
                format!("({})", &test_case.input),
                "expected program to be `{}`, got=`{}`",
                format!("({})", &test_case.input),
                program.to_string().trim_end_matches(";")
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
                program.to_string().trim_end_matches(";"),
                format!("({})", test_case.input),
                "expected program to be `{}`, got=`{}`",
                format!("({})", test_case.input),
                program.to_string().trim_end_matches(";")
            )
        }
    }

    #[test]
    fn should_parse_if_expression() {
        let input = "if (x < y) { x + 1 };";
        let mut parser = Parser::from_str(input);
        let program = parser.parse_program();
        check_parse_errors(&parser);
        validate_program_length(&program, 1);

        let expr = get_and_assert_expression(&program.statements[0]);
        let if_expr = get_and_assert_if_expression(expr);

        let left: Box<dyn Any> = Box::new("x");
        let right: Box<dyn Any> = Box::new("y");
        validate_infix_expression(&if_expr.condition, &left, "<".to_string(), &right);

        assert_eq!(
            if_expr.consequence.statements.len(),
            1,
            "expected `{}` consequence statements, got=`{}`",
            1,
            if_expr.consequence.statements.len()
        );

        let expr = get_and_assert_expression(&if_expr.consequence.statements[0]);
        get_and_assert_infix_expression(expr);

        let left: Box<dyn Any> = Box::new("x");
        let right: Box<dyn Any> = Box::new(1);
        validate_infix_expression(&expr, &left, "+".to_string(), &right);

        assert!(
            if_expr.alternative.is_none(),
            "expected `if expression`'s alternative to be none, got=`{}`",
            if_expr.alternative.as_ref().unwrap().to_string()
        );
    }

    #[test]
    fn should_parse_if_else_expression() {
        let input = "if (x < y) { x + 1 } else { 1 };";
        let mut parser = Parser::from_str(input);
        let program = parser.parse_program();
        check_parse_errors(&parser);
        validate_program_length(&program, 1);

        let expr = get_and_assert_expression(&program.statements[0]);
        let if_expr = get_and_assert_if_expression(expr);

        let left: Box<dyn Any> = Box::new("x");
        let right: Box<dyn Any> = Box::new("y");
        validate_infix_expression(&if_expr.condition, &left, "<".to_string(), &right);

        assert_eq!(if_expr.consequence.statements.len(), 1);

        let expr = get_and_assert_expression(&if_expr.consequence.statements[0]);
        get_and_assert_infix_expression(expr);

        let left: Box<dyn Any> = Box::new("x");
        let right: Box<dyn Any> = Box::new(1);
        validate_infix_expression(&expr, &left, "+".to_string(), &right);

        assert!(
            if_expr.alternative.is_some(),
            "expected `if expression`'s alternative to be not none"
        );
        let alternative = if_expr.alternative.as_ref().unwrap();

        assert_eq!(alternative.statements.len(), 1);

        let expr = get_and_assert_expression(&alternative.statements[0]);
        get_and_assert_integer_literal(expr);
        validate_integer_literal(expr, 1);
    }

    #[test]
    fn should_parse_function_literal() {
        let input = "fn(x, y) { x + y; }";
        let mut parser = Parser::from_str(&input);
        let program = parser.parse_program();
        check_parse_errors(&parser);
        validate_program_length(&program, 1);

        let expr = get_and_assert_expression(&program.statements[0]);
        let function = get_and_assert_function_literal(expr);

        // Validate function's parameters.
        // There should be two identifiers: `x` and `y`.
        let expected_params = Vec::from(["x", "y"])
            .iter()
            .map(|&s| s.to_string())
            .collect();
        validate_function_parameters(&function.parameters, &expected_params);

        // Validate function's body. There should be one infix statement.
        assert_eq!(
            function.body.statements.len(),
            1,
            "expected function's body to have `1` statement, got=`{}`",
            function.body.statements.len()
        );

        let expr = get_and_assert_expression(&function.body.statements[0]);
        let left: Box<dyn Any> = Box::new("x");
        let right: Box<dyn Any> = Box::new("y");
        validate_infix_expression(expr, &left, "+".to_string(), &right);
    }

    #[test]
    fn should_parse_function_parameters() {
        struct TestCase {
            input: String,
            expected_params: Vec<String>,
        }

        impl TestCase {
            fn new(input: &str, expected_params: Vec<&str>) -> Self {
                Self {
                    input: input.to_string(),
                    expected_params: expected_params.iter().map(|&s| s.to_string()).collect(),
                }
            }
        }

        let test_cases = Vec::from([
            TestCase::new("fn() { x };", Vec::new()),
            TestCase::new("fn(x) { x };", Vec::from(["x"])),
            TestCase::new("fn(x, y, z) { x };", Vec::from(["x", "y", "z"])),
            TestCase::new(
                "fn(my_var, another_var) {};",
                Vec::from(["my_var", "another_var"]),
            ),
            TestCase::new(
                "fn(MyVar, AnotherVar) {};",
                Vec::from(["MyVar", "AnotherVar"]),
            ),
        ]);

        for test_case in test_cases {
            let mut parser = Parser::from_str(&test_case.input);
            let program = parser.parse_program();
            check_parse_errors(&parser);

            let expr = get_and_assert_expression(&program.statements[0]);
            let function = get_and_assert_function_literal(expr);

            validate_function_parameters(&function.parameters, &test_case.expected_params);
        }
    }

    #[test]
    fn should_parse_call_expression_starting_with_literal() {
        let input = "add(1, 2 * 3, 4 + 5)";
        let mut parser = Parser::from_str(input);
        let program = parser.parse_program();
        check_parse_errors(&parser);
        validate_program_length(&program, 1);

        let expr = get_and_assert_expression(&program.statements[0]);
        let call_expr = get_and_assert_call_expression(expr);

        validate_identifier_expression(&call_expr.function, "add");
        validate_argument_list_length(call_expr.arguments.len(), 3);

        let val: Box<dyn Any> = Box::new(1);
        validate_literal_expression(&call_expr.arguments[0], &val);

        let left: Box<dyn Any> = Box::new(2);
        let right: Box<dyn Any> = Box::new(3);
        validate_infix_expression(&call_expr.arguments[1], &left, "*".to_string(), &right);

        let left: Box<dyn Any> = Box::new(4);
        let right: Box<dyn Any> = Box::new(5);
        validate_infix_expression(&call_expr.arguments[2], &left, "+".to_string(), &right);
    }

    #[test]
    fn should_parse_call_expression_strting_with_function_literal() {
        let input = "fn(a, b) { a + b }(1, 2)";
        let mut parser = Parser::from_str(input);
        let program = parser.parse_program();
        check_parse_errors(&parser);
        validate_program_length(&program, 1);

        let expr = get_and_assert_expression(&program.statements[0]);
        let call_expr = get_and_assert_call_expression(expr);

        let function = get_and_assert_function_literal(&call_expr.function);
        validate_function_parameters(
            &function.parameters,
            &Vec::from(["a".to_string(), "b".to_string()]),
        );

        validate_argument_list_length(call_expr.arguments.len(), 2);

        let val: Box<dyn Any> = Box::new(1);
        validate_literal_expression(&call_expr.arguments[0], &val);

        let val: Box<dyn Any> = Box::new(2);
        validate_literal_expression(&call_expr.arguments[1], &val);
    }

    #[test]
    fn should_parse_call_expression_with_empty_argument_list() {
        let input = "print()";
        let mut parser = Parser::from_str(input);
        let program = parser.parse_program();
        check_parse_errors(&parser);
        validate_program_length(&program, 1);

        let expr = get_and_assert_expression(&program.statements[0]);
        let call_expr = get_and_assert_call_expression(expr);

        validate_identifier_expression(&call_expr.function, "print");
        validate_argument_list_length(call_expr.arguments.len(), 0);
    }

    #[test]
    fn should_parser_call_expression_with_arguments_that_themselves_are_call_expressions() {
        let input = "add(subtract(1, 99.11), fn(x, y) {x + y}(1, 2))";
        let mut parser = Parser::from_str(input);
        let program = parser.parse_program();
        check_parse_errors(&parser);
        validate_program_length(&program, 1);

        let expr = get_and_assert_expression(&program.statements[0]);
        let call_expr = get_and_assert_call_expression(expr);
        validate_identifier_expression(&call_expr.function, "add");
        validate_argument_list_length(call_expr.arguments.len(), 2);

        // validate add(1, 2) call expression
        let call_expr_arg_1 = get_and_assert_call_expression(&call_expr.arguments[0]);
        validate_identifier_expression(&call_expr_arg_1.function, "subtract");
        validate_argument_list_length(call_expr_arg_1.arguments.len(), 2);
        validate_integer_literal(&call_expr_arg_1.arguments[0], 1);
        validate_float_literal(&call_expr_arg_1.arguments[1], 99.11);

        // validate fn(x, y) {x + y}(1, 2)
        let call_expr_arg_2 = get_and_assert_call_expression(&call_expr.arguments[1]);
        validate_argument_list_length(call_expr_arg_2.arguments.len(), 2);
        let function = get_and_assert_function_literal(&call_expr_arg_2.function);
        validate_function_parameters(
            &function.parameters,
            &Vec::from(["x".to_string(), "y".to_string()]),
        );
        let left: Box<dyn Any> = Box::new("x");
        let right: Box<dyn Any> = Box::new("y");
        let body_expr = get_and_assert_expression(&function.body.statements[0]);
        validate_infix_expression(&body_expr, &left, "+".to_string(), &right);
        validate_integer_literal(&call_expr_arg_2.arguments[0], 1);
        validate_integer_literal(&call_expr_arg_2.arguments[1], 2);
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
        validate_identifier_expression(&expr, "foobar");
        assert_eq!(program.to_string().trim_end_matches(";"), "foobar");
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
        assert_eq!(program.to_string().trim_end_matches(";"), "5");
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
