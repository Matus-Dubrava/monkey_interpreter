use monkey_interpreter::ast::*;
use monkey_interpreter::parser::Parser;

use std::any::Any;

pub fn check_parse_errors(parser: &Parser) {
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

pub fn validate_let_statement(stmt: &Box<dyn Statement>, name: &String) {
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

pub fn validate_return_statement(stmt: &Box<dyn Statement>) {
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

pub fn validate_integer_literal(int_literal: &Box<dyn Expression>, value: i64) {
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

pub fn validate_literal_expression(expression: &Box<dyn Expression>, expected: &Box<dyn Any>) {
    // each literal needs to be registered here before we can
    // test it in infix expression
    let exp = expected.downcast_ref::<i32>();
    if let Some(exp) = exp {
        validate_integer_literal(&expression, *exp as i64);
    }

    let exp = expected.downcast_ref::<i64>();
    if let Some(exp) = exp {
        validate_integer_literal(&expression, *exp);
    }

    let exp = expected.downcast_ref::<String>();
    if let Some(exp) = exp {
        validate_identifier(&expression, exp);
    }

    let exp = expected.downcast_ref::<bool>();
    if let Some(exp) = exp {
        validate_boolean_literal(expression, exp);
    }
}

pub fn validate_boolean_literal(expression: &Box<dyn Expression>, value: &bool) {
    let b = expression.as_any().downcast_ref::<Boolean>();
    assert!(b.is_some(), "expected Expression to be Boolean");
    let b = b.unwrap();

    assert_eq!(
        b.value, *value,
        "expected boolean value to be {}, got={}",
        value, b.value
    );

    assert_eq!(
        b.token_literal(),
        value.to_string(),
        "expected boolean literal to be {}, got={}",
        value.to_string(),
        b.token_literal()
    );
}

pub fn validate_identifier(expression: &Box<dyn Expression>, value: &str) {
    let ident = expression.as_any().downcast_ref::<Identifier>();
    assert!(ident.is_some(), "expected Expression to be Identifier");
    let ident = ident.unwrap();
    assert_eq!(
        ident.value, value,
        "exprected Identifier value to be {}, got={}",
        value, ident.value
    );
    assert_eq!(
        ident.token_literal(),
        value,
        "expected Identifier token literal to be {}, got={}",
        value,
        ident.token_literal()
    );
}

pub fn validate_infix_expression(
    expression: &Box<dyn Expression>,
    left: &Box<dyn Any>,
    operator: String,
    right: &Box<dyn Any>,
) {
    let expr = expression.as_any().downcast_ref::<InfixExpression>();
    assert!(expr.is_some(), "Expression is not InfixExpression");
    let expr = expr.unwrap();

    validate_literal_expression(&expr.left, &left);
    assert_eq!(
        expr.operator, operator,
        "expected operator `{}`, got={}",
        operator, expr.operator
    );
    validate_literal_expression(&expr.right, &right);
}
