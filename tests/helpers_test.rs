mod helpers;

// this module contains tests to validate correctness
// of helper functions such as validate_boolean_literal etc.
#[cfg(test)]
mod helpers_test {
    use std::any::Any;

    use monkey_interpreter::ast::*;
    use monkey_interpreter::lexer::Lexer;
    use monkey_interpreter::parser::Parser;
    use monkey_interpreter::token::{Token, TokenType};

    use crate::helpers::*;

    #[test]
    fn test_validate_infix_expression_helper() {
        // testing integer literals
        let input = "1 + 2";
        let lex = Lexer::new(&input.to_string());
        let mut parser = Parser::new(lex);
        let program = parser.parse_program().unwrap();

        let expr = get_and_assert_expression(&program.statements[0]);
        let left: Box<dyn Any> = Box::new(1);
        let right: Box<dyn Any> = Box::new(2);
        validate_infix_expression(&expr, &left, "+".to_string(), &right);

        // testing boolean literals
        let input = "true + false";
        let lex = Lexer::new(&input.to_string());
        let mut parser = Parser::new(lex);
        let program = parser.parse_program().unwrap();

        let expr = get_and_assert_expression(&program.statements[0]);
        get_and_assert_infix_expression(&expr);

        let left: Box<dyn Any> = Box::new(true);
        let right: Box<dyn Any> = Box::new(false);
        validate_infix_expression(&expr, &left, "+".to_string(), &right)
    }

    #[test]
    fn test_validate_identifier_helper() {
        let input = "x;";
        let lex = Lexer::new(&input.to_string());
        let mut parser = Parser::new(lex);
        let program = parser.parse_program().unwrap();

        assert_eq!(program.statements.len(), 1);
        let expr = get_and_assert_expression(&program.statements[0]);
        validate_identifier(&expr, "x");
    }

    #[test]
    fn test_validate_boolean_helper() {
        let expr: Box<dyn Expression> = Box::new(Boolean::new(
            Token::from_str(TokenType::TRUE, "true".to_string()),
            true,
        ));

        validate_boolean_literal(&expr, &true);

        let expr: Box<dyn Expression> = Box::new(Boolean::new(
            Token::from_str(TokenType::FALSE, "false".to_string()),
            false,
        ));

        validate_boolean_literal(&expr, &false);

        let input = "true";
        let lex = Lexer::new(&input.to_string());
        let mut parser = Parser::new(lex);
        let program = parser.parse_program().unwrap();

        let expr = get_and_assert_expression(&program.statements[0]);
        validate_boolean_literal(&expr, &true);
    }

    #[test]
    fn test_validate_integer_literal_helper() {
        let int_literal: Box<dyn Expression> = Box::new(IntegerLiteral::new(
            Token::from_str(TokenType::INT, "5".to_string()),
            5,
        ));

        validate_integer_literal(&int_literal, 5);
    }
}
