use monkey_interpreter::ast::*;
use monkey_interpreter::parser::Parser;

use std::any::Any;

/// expects Statement
/// checks whether provided statement is ExpressionStatement
/// and returs Expression that is stored within ExpressionStatement
/// or panics if the provided statement is not ExpressionStatement
pub fn get_and_assert_expression(stmt: &Box<dyn Statement>) -> &Box<dyn Expression> {
    let expr_statement = stmt.as_any().downcast_ref::<ExpressionStatement>();
    assert!(
        expr_statement.is_some(),
        "expected statement to be ExpressionStatement"
    );

    return &expr_statement.unwrap().expression;
}

pub fn get_and_assert_infix_expression(expr: &Box<dyn Expression>) -> &InfixExpression {
    let infix_expr = expr.as_any().downcast_ref::<InfixExpression>();
    assert!(
        infix_expr.is_some(),
        "expected expression to be InfixExpression"
    );

    return infix_expr.unwrap();
}

pub fn get_and_assert_prefix_expression(expr: &Box<dyn Expression>) -> &PrefixExpression {
    let prefix_expr = expr.as_any().downcast_ref::<PrefixExpression>();
    assert!(
        prefix_expr.is_some(),
        "expected expression to be InfixExpression"
    );

    return prefix_expr.unwrap();
}

pub fn get_and_assert_boolean(expr: &Box<dyn Expression>) -> &Boolean {
    let boolean = expr.as_any().downcast_ref::<Boolean>();
    assert!(boolean.is_some(), "expected expression to be Boolean");
    return boolean.unwrap();
}

pub fn get_and_assert_identifier(expr: &Box<dyn Expression>) -> &Identifier {
    let identifier = expr.as_any().downcast_ref::<Identifier>();
    assert!(identifier.is_some(), "expected expression to be Identifier");
    return identifier.unwrap();
}

pub fn get_and_assert_integer_literal(expr: &Box<dyn Expression>) -> &IntegerLiteral {
    let int_literal = expr.as_any().downcast_ref::<IntegerLiteral>();
    assert!(
        int_literal.is_some(),
        "expected expression to be IntegerLiteral"
    );
    return int_literal.unwrap();
}

pub fn get_and_assert_let_statement(stmt: &Box<dyn Statement>) -> &LetStatement {
    let let_stmt = stmt.as_any().downcast_ref::<LetStatement>();
    assert!(let_stmt.is_some(), "expected expression to be LetStatement");
    return let_stmt.unwrap();
}

pub fn get_and_assert_return_statement(stmt: &Box<dyn Statement>) -> &ReturnStatement {
    let return_stmt = stmt.as_any().downcast_ref::<ReturnStatement>();
    assert!(
        return_stmt.is_some(),
        "expected expression to be ReturnStatement"
    );
    return return_stmt.unwrap();
}

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

pub fn validate_program_length(program: &Program, len: usize) {
    assert_eq!(
        program.statements.len(),
        len,
        "expected program to have `{}` statements, got={}",
        len,
        program.statements.len()
    )
}

pub fn validate_let_statement(stmt: &Box<dyn Statement>, name: &String) {
    let let_stmt = get_and_assert_let_statement(stmt);

    assert_eq!(
        stmt.token_literal(),
        "let",
        "expected token literal to be `let`, got={}",
        stmt.token_literal()
    );

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
    get_and_assert_return_statement(&stmt);

    assert_eq!(
        stmt.token_literal(),
        "return",
        "statement's token literal is not 'return', got={}",
        stmt.token_literal()
    );
}

pub fn validate_integer_literal(expr: &Box<dyn Expression>, value: i64) {
    let int_literal = get_and_assert_integer_literal(&expr);

    assert_eq!(
        int_literal.value, value,
        "expected value to be {}, got={}",
        int_literal.value, value
    );

    assert_eq!(
        int_literal.token_literal(),
        value.to_string(),
        "expected token literal to be {}, got={}",
        int_literal.token_literal(),
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

pub fn validate_boolean_literal(expr: &Box<dyn Expression>, value: &bool) {
    let boolean = get_and_assert_boolean(&expr);

    assert_eq!(
        boolean.value, *value,
        "expected boolean value to be {}, got={}",
        value, boolean.value
    );

    assert_eq!(
        boolean.token_literal(),
        value.to_string(),
        "expected boolean literal to be {}, got={}",
        value.to_string(),
        boolean.token_literal()
    );
}

pub fn validate_identifier(expr: &Box<dyn Expression>, value: &str) {
    let ident = get_and_assert_identifier(&expr);
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
    expr: &Box<dyn Expression>,
    left: &Box<dyn Any>,
    operator: String,
    right: &Box<dyn Any>,
) {
    let expr = get_and_assert_infix_expression(&expr);
    validate_literal_expression(&expr.left, &left);
    validate_operator(expr.operator.clone(), operator);
    validate_literal_expression(&expr.right, &right);
}

pub fn validate_prefix_expression(
    expr: &Box<dyn Expression>,
    operator: String,
    right: &Box<dyn Any>,
) {
    let expr = get_and_assert_prefix_expression(&expr);
    validate_operator(expr.operator.clone(), operator);
    validate_literal_expression(&expr.right, &right);
}

pub fn validate_operator(operator: String, expected_operator: String) {
    assert_eq!(
        expected_operator, operator,
        "expected operator `{}`, got={}",
        expected_operator, operator
    );
}
