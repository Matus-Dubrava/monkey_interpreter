#[cfg(test)]
mod evaluator_test {
    use monkey_interpreter::ast::Node;
    use monkey_interpreter::object::Object;
    use monkey_interpreter::parser::Parser;

    #[test]
    fn should_evaluate_integer_expression() {
        let test_cases = vec![
            ("5", 5),
            ("10", 10),
            ("-5", -5),
            ("-10", -10),
            ("0", 0),
            ("-0", 0),
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
        let test_cases = vec![("true", true), ("false", false)];

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
}
