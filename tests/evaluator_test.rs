#[cfg(test)]
mod evaluator_test {
    use monkey_interpreter::ast::Node;
    use monkey_interpreter::object::Object;
    use monkey_interpreter::parser::Parser;

    #[test]
    fn should_be_able_to_handle_errors() {
        let test_cases = vec![
            ("5 + true;", "type mismatch: INTEGER + BOOLEAN"),
            ("5 + true; 5;", "type mismatch: INTEGER + BOOLEAN"),
            ("-true", "unknown operator: -BOOLEAN"),
            ("true + false;", "unknown operator: BOOLEAN + BOOLEAN"),
            ("5; true + false; 5", "unknown operator: BOOLEAN + BOOLEAN"),
            (
                "if (10 > 1) { true + false; }",
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
            (
                "
                if (10 > 1) {
                    if (10 > 1) {
                        return true + false;
                    }
                    return 1;
                }
            ",
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
            (
                "
                if (10 > 1) {
                    if (10 > 1) {
                        true + false;
                    }
                    return 1;
                }
            ",
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
        ];

        for test_case in test_cases {
            let mut parser = Parser::from_str(test_case.0);
            let program = parser.parse_program();
            let evaluated = program.eval();

            assert!(
                evaluated.is_some(),
                "Expected program to evaluate to a value, got=`None`"
            );

            test_error(evaluated.unwrap(), test_case.1);
        }
    }

    #[test]
    fn should_evaluate_return_statement() {
        let test_cases = vec![
            ("return 10;", 10),
            ("return 10; 9;", 10),
            ("return 2 * 5; 9;", 10),
            ("9; return 2 * 5; 9;", 10),
            (
                "
                if (10 > 1) {
                    if (10 > 1) {
                        return 10;
                    }
                    return 1;
                }",
                10,
            ),
        ];

        for test_case in test_cases {
            let mut parser = Parser::from_str(test_case.0);
            let program = parser.parse_program();
            let evaluated = program.eval();

            assert!(
                evaluated.is_some(),
                "Expected block statement `{}` to be evaluated to value, got=`None`",
                test_case.0
            );

            test_integer_object(evaluated.unwrap(), test_case.1);
        }
    }

    #[test]
    fn should_evaluate_if_expression() {
        let test_cases = vec![
            ("if (true) { 10 }", Some(10)),
            ("if (false) { 10 }", None),
            ("if (1) { 10 }", Some(10)),
            ("if (1 < 2) { 10 }", Some(10)),
            ("if (1 > 2) { 10 }", None),
            ("if (1 > 2) { 10 } else { 20 }", Some(20)),
            ("if (1 < 2) { 10 } else { 20 }", Some(10)),
        ];

        for test_case in test_cases {
            let mut parser = Parser::from_str(test_case.0);
            let program = parser.parse_program();
            let evaluated = program.eval();

            assert!(
                evaluated.is_some(),
                "Expected If expression `{}` to be evaluated to a value, got=`None`",
                test_case.0,
            );

            match test_case.1 {
                None => test_null_object(evaluated.unwrap()),
                Some(val) => test_integer_object(evaluated.unwrap(), val),
            }
        }
    }

    #[test]
    fn should_evaluate_integer_expression() {
        let test_cases = vec![
            ("5", 5),
            ("10", 10),
            ("-5", -5),
            ("-10", -10),
            ("0", 0),
            ("-0", 0),
            ("5 + 5 + 5 + 5 - 10", 10),
            ("2 * 2 * 2 * 2 * 2", 32),
            ("-50 + 100 + -50", 0),
            ("5 * 2 + 10", 20),
            ("5 + 2 * 10", 25),
            ("20 + 2 * -10", 0),
            ("50 / 2 * 2 + 10", 60),
            ("2 * (5 + 10)", 30),
            ("3 * 3 * 3 + 10", 37),
            ("3 * (3 * 3) + 10", 37),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
            ("2 + 3 * (4 - 1)", 11),
            ("(2 + 3) * (4 - 1)", 15),
            ("(10 - 2) * 3 / 2 + (8 / 4)", 14),
            ("(2 * 3) + (-4 / 2) + 5", 9),
            ("1 + 2 * 3 - 4 / 2", 5),
            ("((5 + 3) * 2) / ((4 - 2) * 3)", 2),
        ];

        for test_case in test_cases {
            let evaluated = test_eval(test_case.0);
            assert!(
                evaluated.is_some(),
                "Expected integer `{}`, got=`None`",
                test_case.0
            );
            test_integer_object(evaluated.unwrap(), test_case.1);
        }
    }

    #[test]
    fn should_evaluate_float_expression() {
        let test_cases = vec![
            ("5.5", 5.5),
            ("10.11", 10.11),
            ("-5.5", -5.5),
            ("-10.123", -10.123),
            ("0.0", 0.0),
            ("-0.0", 0.0),
            ("5.0", 5.0),
            ("10.0", 10.0),
            ("-5.0", -5.0),
            ("-10.0", -10.0),
            ("0.0", 0.0),
            ("-0.0", 0.0),
            ("5.0 + 5.0 + 5.0 + 5.0 - 10.0", 10.0),
            ("2.0 * 2.0 * 2.0 * 2.0 * 2.0", 32.0),
            ("-50.0 + 100.0 + -50.0", 0.0),
            ("5.0 * 2.0 + 10.0", 20.0),
            ("5.0 + 2.0 * 10.0", 25.0),
            ("20.0 + 2.0 * -10.0", 0.0),
            ("50.0 / 2.0 * 2.0 + 10.0", 60.0),
            ("2.0 * (5.0 + 10.0)", 30.0),
            ("3.0 * 3.0 * 3.0 + 10.0", 37.0),
            ("3.0 * (3.0 * 3.0) + 10.0", 37.0),
            ("(5.0 + 10.0 * 2.0 + 15.0 / 3.0) * 2.0 + -10.0", 50.0),
        ];

        for test_case in test_cases {
            let evaluated = test_eval(test_case.0);
            assert!(
                evaluated.is_some(),
                "Expected integer `{}`, got=`None`",
                test_case.0
            );
            test_float_object(evaluated.unwrap(), test_case.1);
        }
    }

    #[test]
    fn should_evalute_boolean_expression() {
        let test_cases = vec![
            ("true", true),
            ("false", false),
            ("1 < 2", true),
            ("1 > 2", false),
            ("1 < 1", false),
            ("1 > 1", false),
            ("1 == 1", true),
            ("1 != 1", false),
            ("1 == 2", false),
            ("1 != 2", true),
            ("1.0 < 2.0", true),
            ("1.0 > 2.0", false),
            ("1.0 < 1.0", false),
            ("1.0 > 1.0", false),
            ("1.0 == 1.0", true),
            ("1.0 != 1.0", false),
            ("1.0 == 2.0", false),
            ("1.0 != 2.0", true),
            ("true == true", true),
            ("false == false", true),
            ("true == false", false),
            ("true != false", true),
            ("false != true", true),
            ("(1 < 2) == true", true),
            ("(1 < 2) == false", false),
            ("(1 > 2) == true", false),
            ("(1 > 2) == false", true),
            ("true == true", true),
            ("false == false", true),
            ("true == false", false),
            ("true != false", true),
            ("false != true", true),
            ("(1.0 < 2.0) == true", true),
            ("(1.0 < 2.0) == false", false),
            ("(1.0 > 2.0) == true", false),
            ("(1.0 > 2.0) == false", true),
        ];

        for test_case in test_cases {
            let evaluated = test_eval(test_case.0);
            assert!(
                evaluated.is_some(),
                "Expected boolean `{}`, got=`None`",
                test_case.0
            );
            test_boolean_object(evaluated.unwrap(), test_case.1);
        }
    }

    #[test]
    fn should_evaluate_bang_operator() {
        let test_cases = vec![
            ("!true", false),
            ("!false", true),
            ("!5", false),
            ("!!true", true),
            ("!!false", false),
            ("!!5", true),
        ];

        for test_case in test_cases {
            let evaluated = test_eval(test_case.0);
            assert!(
                evaluated.is_some(),
                "Expected expression {} to evaluate to value.",
                test_case.0
            );
            test_boolean_object(evaluated.unwrap(), test_case.1);
        }
    }

    fn test_error(obj: Object, expected_msg: &str) {
        match obj {
            Object::Error(msg) if msg == expected_msg => (),
            Object::Error(msg) => panic!(
                "Expected Error message to be=`{}`, got=`{}`",
                expected_msg, msg
            ),
            _ => panic!("Expected Error object, got=`{}`", obj.to_string()),
        }
    }

    fn test_eval(input: &str) -> Option<Object> {
        let mut parser = Parser::from_str(input);
        let program: Box<dyn Node> = Box::new(parser.parse_program());
        return program.eval();
    }

    fn test_integer_object(obj: Object, expected: i64) {
        match obj {
            Object::Integer(val) => assert_eq!(
                val, expected,
                "Integer value doesn't match. Expected=`{}`, got=`{}`",
                expected, val
            ),
            _ => panic!("Expected Integer, got=`{}`", obj.to_string()),
        }
    }

    fn test_float_object(obj: Object, expected: f64) {
        match obj {
            Object::Float(val) => assert_eq!(
                val, expected,
                "Float value doesn't match. Expected=`{}`, got=`{}`",
                expected, val
            ),
            _ => panic!("Expected Float, got=`{}`", obj.to_string()),
        }
    }

    fn test_boolean_object(obj: Object, expected: bool) {
        match obj {
            Object::Boolean(val) => assert_eq!(
                val, expected,
                "Boolean value doesn't match. Expected=`{}`, got=`{}`",
                expected, val
            ),
            _ => panic!("Expected Boolean, got=`{}`", obj.to_string()),
        }
    }

    fn test_null_object(obj: Object) {
        match obj {
            Object::Null => (),
            _ => panic!("Expected Null, got=`{}`", obj.to_string()),
        }
    }
}
