#[cfg(test)]
mod evaluator_test {
    use monkey_interpreter::ast::Node;
    use monkey_interpreter::evaluator::eval;
    use monkey_interpreter::object::Object;
    use monkey_interpreter::parser::Parser;

    #[test]
    fn should_evaluate_integer_expression() {
        let test_cases = vec![("5", 5), ("10", 10)];

        for test_case in test_cases {
            let evaluated = test_eval(test_case.0);
            test_integer_object(evaluated, test_case.1);
        }
    }

    fn test_eval(input: &str) -> Object {
        let mut parser = Parser::from_str(input);
        let program: Box<dyn Node> = Box::new(parser.parse_program());

        return eval(program);
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
}
