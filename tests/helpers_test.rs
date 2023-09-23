mod helpers;

// this module contains tests to validate correctness
// of helper functions such as validate_boolean_literal etc.
#[cfg(test)]
mod helpers_test {
    use std::any::Any;

    use monkey_interpreter::ast::*;
    use monkey_interpreter::parser::Parser;
    use monkey_interpreter::token::{Token, TokenType};

    use crate::helpers::*;

    #[test]
    fn test_validate_infix_expression_helper() {
        // testing integer literals
        let input = "1 + 2";
        let mut parser = Parser::from_str(input);
        let program = parser.parse_program();
        validate_program_length(&program, 1);

        let expr = get_and_assert_expression(&program.statements[0]);
        let left: Box<dyn Any> = Box::new(1);
        let right: Box<dyn Any> = Box::new(2);
        validate_infix_expression(&expr, &left, "+".to_string(), &right);

        // testing boolean literals
        let input = "true + false";
        let mut parser = Parser::from_str(input);
        let program = parser.parse_program();

        let expr = get_and_assert_expression(&program.statements[0]);
        get_and_assert_infix_expression(&expr);

        let left: Box<dyn Any> = Box::new(true);
        let right: Box<dyn Any> = Box::new(false);
        validate_infix_expression(&expr, &left, "+".to_string(), &right)
    }

    #[test]
    fn test_validate_identifier_helper() {
        let input = "x;";
        let mut parser = Parser::from_str(input);
        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 1);
        let expr = get_and_assert_expression(&program.statements[0]);
        validate_identifier(&expr, "x");
    }

    #[test]
    fn test_validate_boolean_helper() {
        let expr: Box<dyn Expression> =
            Box::new(Boolean::new(Token::from_str(TokenType::TRUE, "true"), true));

        validate_boolean_literal(&expr, &true);

        let expr: Box<dyn Expression> = Box::new(Boolean::new(
            Token::from_str(TokenType::FALSE, "false"),
            false,
        ));

        validate_boolean_literal(&expr, &false);

        let input = "true";
        let mut parser = Parser::from_str(input);
        let program = parser.parse_program();

        let expr = get_and_assert_expression(&program.statements[0]);
        validate_boolean_literal(&expr, &true);
    }

    #[test]
    fn test_validate_integer_literal_helper() {
        let value = 12345;
        let int_literal: Box<dyn Expression> = Box::new(IntegerLiteral::new(
            Token::from_str(TokenType::INT, &value.to_string()),
            value,
        ));

        validate_integer_literal(&int_literal, value);
    }

    #[test]
    fn test_validate_float_literal_helper() {
        let value = 592.123;
        let float_literal: Box<dyn Expression> = Box::new(FloatLiteral::new(
            Token::from_str(TokenType::FLOAT, &value.to_string()),
            value,
        ));

        validate_float_literal(&float_literal, value);
    }
}
