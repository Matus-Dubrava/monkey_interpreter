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

pub fn get_and_assert_float_literal(expr: &Box<dyn Expression>) -> &FloatLiteral {
    let float_literal = expr.as_any().downcast_ref::<FloatLiteral>();
    assert!(
        float_literal.is_some(),
        "expected expression to be FloatLiteral"
    );
    return float_literal.unwrap();
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

pub fn get_and_assert_if_expression(expr: &Box<dyn Expression>) -> &IfExpression {
    let if_expr = expr.as_any().downcast_ref::<IfExpression>();
    assert!(if_expr.is_some(), "expected expression to be IfExpression");
    return if_expr.unwrap();
}

pub fn get_and_assert_function_literal(expr: &Box<dyn Expression>) -> &FunctionLiteral {
    let fn_literal = expr.as_any().downcast_ref::<FunctionLiteral>();
    assert!(fn_literal.is_some(), "expected expression to be FunctionLiteral");
    return fn_literal.unwrap();
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
        "statement's token literal is not 'return', got=`{}`",
        stmt.token_literal()
    );
}

pub fn validate_integer_literal(expr: &Box<dyn Expression>, value: i64) {
    let int_literal = get_and_assert_integer_literal(&expr);

    assert_eq!(
        int_literal.value, value,
        "expected value to be `{}`, got=`{}`",
        int_literal.value, value
    );

    assert_eq!(
        int_literal.token_literal(),
        value.to_string(),
        "expected token literal to be `{}`, got=`{}`",
        int_literal.token_literal(),
        value.to_string()
    );
}


pub fn validate_float_literal(expr: &Box<dyn Expression>, value: f64) {
    // Note about validating float literals. Rust doesn't store extra 
    // 0's after decimal point. Meaning that 0.0, 0.00, 0.000 etc will 
    // all be turned into 0. Similarly 1.100 is turned into 1.1. When 
    // dealing with these numbers, first assertion that tests actual value
    // will pass because both sides are trimmed but the second assertion
    // will fail because `token_literal` is stored as string during tokenizing
    // phase and as such is never trimmed while the `expected value` is. 
    // Either don't test against floats that end with explicit 0's or 
    // update this function to handle such case.
    let float_literal = get_and_assert_float_literal(&expr);

    assert_eq!(
        float_literal.value, value,
        "expected value to be `{}`, got=`{}`",
        value, float_literal.value
    );

    assert_eq!(
        float_literal.token_literal(),
        value.to_string(),
        "exected token literal to be `{}`, got=`{}`",
        value.to_string(),
        float_literal.token_literal()
    );
}

/// Validates any type of literal by trying to cast the 
/// expected value into any known type and calls appropriate
/// valudation function assocaited with that concrete type.
/// If this process can't cast the expected value into any
/// know type, it panics.
/// Note: any new literal type needs to be registered here
/// before it can be tested in prefix/infix expressions tests.
pub fn validate_literal_expression(expression: &Box<dyn Expression>, expected: &Box<dyn Any>) {
    let mut is_known_literal = false;

    let exp = expected.downcast_ref::<i32>();
    if let Some(exp) = exp {
        validate_integer_literal(&expression, *exp as i64);
        is_known_literal = true;
    }

    let exp = expected.downcast_ref::<i64>();
    if let Some(exp) = exp {
        validate_integer_literal(&expression, *exp);
        is_known_literal = true;
    }

    let exp = expected.downcast_ref::<f64>();
    if let Some(exp) = exp {
        validate_float_literal(&expression, *exp);
        is_known_literal = true;
    }

    let exp = expected.downcast_ref::<bool>();
    if let Some(exp) = exp {
        validate_boolean_literal(expression, exp);
        is_known_literal = true;
    }

    // Note on why there there are two downcasts for strings.
    // We are handling both cases when expected value is passed 
    // either String or &str. Both of these are valid represenations,
    // and both will be treated as if the caller want to validate
    // Identifier.
    let exp = expected.downcast_ref::<String>();
    if let Some(exp) = exp {
        validate_identifier_expression(&expression, exp);
        is_known_literal = true;
    }

    let exp = expected.downcast_ref::<&str>();
    if let Some(exp) = exp {
        validate_identifier_expression(&expression, exp);
        is_known_literal = true;
    }


    assert!(is_known_literal, 
        "Provided literal's type is not known. Received expression=`{}`. This type might not have been registered yet.",
        expression.to_string()
    );
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

/// The difference between `validate_identifier_expression` and 
/// `validate_identifier` is that one the format expects `Expression` trait
/// that holds an `Identifier` while the later expects concerete `Identifier`.
/// In most cases, `validate_identifier_expression` is enough but 
/// `FunctionLiteral` stores it's parameters directly in a `Vec<Identifier>`
/// and we would need to do some shenanigans to upcast it to `Expression`
/// if we wanted to use this function. Therefore there are two versions 
/// of this function implemented here.
pub fn validate_identifier_expression(expr: &Box<dyn Expression>, value: &str) {
    let ident = get_and_assert_identifier(&expr);
    _validate_identifier(ident, value)
}

pub fn validate_identifier(ident: &Identifier, value: &str) {
    _validate_identifier(ident, value)
}

pub fn _validate_identifier(ident: &Identifier, value: &str) {
    assert_eq!(
        ident.value, value,
        "exprected Identifier value to be `{}`, got=`{}`",
        value, ident.value
    );
    assert_eq!(
        ident.token_literal(),
        value,
        "expected Identifier token literal to be `{}`, got=`{}`",
        value,
        ident.token_literal()
    );
}

pub fn validate_function_parameters(parameters: &Vec<Identifier>, expected_parameters: &Vec<String>) {
    assert_eq!(parameters.len(), expected_parameters.len(), 
        "Expected `{}` functions parameters, got=`{}`",
        expected_parameters.len(), parameters.len()
    );

    for (param, expected_param) in parameters.iter().zip(expected_parameters.iter()) {
        validate_identifier(param, &expected_param);
    }
}