use crate::eval::is_truthy;
use crate::object::Object;
use crate::token::Token;

use std::any::Any;

pub trait Node {
    fn token_literal(&self) -> &str;
    fn to_string(&self) -> String;
    fn eval(&self) -> Option<Object>;
}

pub trait Statement: Node {
    fn statement_node(&self);
    fn as_any(&self) -> &dyn Any;
}

pub trait Expression: Node {
    fn expression_node(&self);
    fn as_any(&self) -> &dyn Any;
}

pub struct DummyExpression;

impl Expression for DummyExpression {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for DummyExpression {
    fn token_literal(&self) -> &str {
        "dummy literal"
    }

    fn to_string(&self) -> String {
        "(dummy expression)".to_string()
    }

    fn eval(&self) -> Option<Object> {
        unimplemented!()
    }
}

pub struct InfixExpression {
    pub token: Token,
    pub left: Box<dyn Expression>,
    pub operator: String,
    pub right: Box<dyn Expression>,
}

impl Expression for InfixExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn expression_node(&self) {}
}

impl Node for InfixExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn to_string(&self) -> String {
        format!(
            "({} {} {})",
            self.left.to_string(),
            self.operator,
            self.right.to_string()
        )
    }

    fn eval(&self) -> Option<Object> {
        let left = self.left.eval().unwrap_or(Object::Null);
        let right = self.right.eval().unwrap_or(Object::Null);

        match left {
            Object::Error(_) => return Some(left),
            _ => (),
        }

        match right {
            Object::Error(_) => return Some(right),
            _ => (),
        }

        match (&left, self.operator.as_str(), &right) {
            (Object::Integer(l), "+", Object::Integer(r)) => Some(Object::Integer(l + r)),
            (Object::Float(l), "+", Object::Float(r)) => Some(Object::Float(l + r)),
            (Object::Integer(l), "-", Object::Integer(r)) => Some(Object::Integer(l - r)),
            (Object::Float(l), "-", Object::Float(r)) => Some(Object::Float(l - r)),
            (Object::Integer(l), "*", Object::Integer(r)) => Some(Object::Integer(l * r)),
            (Object::Float(l), "*", Object::Float(r)) => Some(Object::Float(l * r)),
            (Object::Integer(l), "/", Object::Integer(r)) => Some(Object::Integer(l / r)),
            (Object::Float(l), "/", Object::Float(r)) => Some(Object::Float(l / r)),
            (Object::Integer(l), "==", Object::Integer(r)) => Some(Object::Boolean(l == r)),
            (Object::Float(l), "==", Object::Float(r)) => Some(Object::Boolean(l == r)),
            (Object::Integer(l), "!=", Object::Integer(r)) => Some(Object::Boolean(l != r)),
            (Object::Float(l), "!=", Object::Float(r)) => Some(Object::Boolean(l != r)),
            (Object::Integer(l), "<", Object::Integer(r)) => Some(Object::Boolean(l < r)),
            (Object::Float(l), "<", Object::Float(r)) => Some(Object::Boolean(l < r)),
            (Object::Integer(l), ">", Object::Integer(r)) => Some(Object::Boolean(l > r)),
            (Object::Float(l), ">", Object::Float(r)) => Some(Object::Boolean(l > r)),
            (Object::Boolean(l), "==", Object::Boolean(r)) => Some(Object::Boolean(l == r)),
            (Object::Boolean(l), "!=", Object::Boolean(r)) => Some(Object::Boolean(l != r)),
            _ => {
                if left.get_type() != right.get_type() {
                    return Some(Object::Error(format!(
                        "type mismatch: {} {} {}",
                        left.get_type(),
                        self.operator,
                        right.get_type()
                    )));
                } else {
                    return Some(Object::Error(format!(
                        "unknown operator: {} {} {}",
                        left.get_type(),
                        self.operator,
                        right.get_type()
                    )));
                }
            }
        }
    }
}

impl InfixExpression {
    pub fn new(
        token: Token,
        left: Box<dyn Expression>,
        operator: &str,
        right: Box<dyn Expression>,
    ) -> Self {
        InfixExpression {
            token,
            left,
            operator: operator.to_string(),
            right,
        }
    }
}

pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<dyn Expression>,
}

impl Expression for PrefixExpression {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn to_string(&self) -> String {
        format!("({}{})", self.operator, self.right.to_string()).to_string()
    }

    fn eval(&self) -> Option<Object> {
        let right = self.right.eval().unwrap_or(Object::Null);

        match right {
            Object::Error(_) => return Some(right),
            _ => (),
        }

        match self.operator.as_str() {
            // Integers: evaluate to `true` unless it is `0`
            // Floats: evalute to `true` unless it is 0.0
            "!" => match right {
                Object::Boolean(val) if val == true => Some(Object::Boolean(false)),
                Object::Boolean(val) if val == false => Some(Object::Boolean(true)),
                Object::Integer(val) if val == 0 => Some(Object::Boolean(true)),
                Object::Integer(_) => Some(Object::Boolean(false)),
                Object::Float(val) if val == 0.0 => Some(Object::Boolean(true)),
                Object::Float(_) => Some(Object::Boolean(false)),
                _ => None,
            },
            "-" => match right {
                Object::Integer(val) => Some(Object::Integer(-val)),
                Object::Float(val) if val == 0.0 => Some(Object::Float(val)),
                Object::Float(val) => Some(Object::Float(-val)),
                _ => Some(Object::Error(format!(
                    "unknown operator: {}{}",
                    self.operator,
                    right.get_type()
                ))),
            },
            _ => Some(Object::Error(format!(
                "unknown operator: {}{}",
                self.operator,
                right.get_type()
            ))),
        }
    }
}

impl PrefixExpression {
    pub fn new(token: Token, operator: &str, right: Box<dyn Expression>) -> Self {
        PrefixExpression {
            token,
            operator: operator.to_string(),
            right,
        }
    }
}

pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Box<dyn Expression>,
}

impl Statement for ExpressionStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn statement_node(&self) {}
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn to_string(&self) -> String {
        self.expression.to_string()
    }

    fn eval(&self) -> Option<Object> {
        self.expression.eval()
    }
}

impl ExpressionStatement {
    pub fn new(token: Token, expression: Box<dyn Expression>) -> Self {
        ExpressionStatement { token, expression }
    }
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> &str {
        match self.statements.len() {
            0 => "",
            _ => self.statements[0].token_literal(),
        }
    }

    fn to_string(&self) -> String {
        self.statements
            .iter()
            .map(|stmt| format!("{}; ", stmt.to_string()))
            .collect::<String>()
            .trim_end()
            .to_string()
    }

    fn eval(&self) -> Option<Object> {
        let mut res: Option<Object> = None;

        for stmt in &self.statements {
            res = stmt.eval();

            match &res {
                None => return None,
                Some(obj) => match obj {
                    // Unwrap the return value end exit execution
                    // of Program. We are handling this differently
                    // when executing BlockStatement where we
                    // need to let the value bulle up to the outermost
                    // block. Check BlockStatement's `eval` to
                    // see the difference.
                    Object::ReturnValue(val) => return Some(val.as_ref().clone()),
                    // Stop execution when we encouter Error object.
                    // BlockStatement's eval propagates error to the
                    // this scope.
                    Object::Error(_) => return Some(obj.clone()),
                    _ => continue,
                },
            }
        }

        return res;
    }
}

impl Program {
    pub fn new() -> Self {
        let statements: Vec<Box<dyn Statement>> = Vec::new();
        Program { statements }
    }

    pub fn from_statements(statements: Vec<Box<dyn Statement>>) -> Self {
        Program { statements }
    }
}

#[derive(Debug)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl Expression for IntegerLiteral {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn to_string(&self) -> String {
        self.token.literal.clone()
    }

    fn eval(&self) -> Option<Object> {
        Some(Object::Integer(self.value))
    }
}

impl IntegerLiteral {
    pub fn new(token: Token, value: i64) -> Self {
        IntegerLiteral { token, value }
    }
}

#[derive(Debug)]
pub struct FloatLiteral {
    pub token: Token,
    pub value: f64,
}

impl Expression for FloatLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn expression_node(&self) {}
}

impl Node for FloatLiteral {
    fn to_string(&self) -> String {
        self.token.literal.clone()
    }

    fn token_literal(&self) -> &str {
        self.token.literal.as_str()
    }

    fn eval(&self) -> Option<Object> {
        Some(Object::Float(self.value))
    }
}

impl FloatLiteral {
    pub fn new(token: Token, value: f64) -> Self {
        Self { token, value }
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Expression for Identifier {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }

    fn eval(&self) -> Option<Object> {
        unimplemented!()
    }
}

impl Identifier {
    pub fn new(token: Token, value: String) -> Self {
        Identifier { token, value }
    }
}

pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Box<dyn Expression>,
}

impl Statement for LetStatement {
    fn statement_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn to_string(&self) -> String {
        format!(
            "{} {} = {}",
            self.token_literal(),
            &self.name.to_string(),
            &self.value.to_string()
        )
    }

    fn eval(&self) -> Option<Object> {
        unimplemented!()
    }
}

impl LetStatement {
    pub fn new(token: Token, name: Identifier, value: Box<dyn Expression>) -> LetStatement {
        LetStatement { token, name, value }
    }
}

pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Box<dyn Expression>,
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn to_string(&self) -> String {
        format!(
            "{} {}",
            self.token_literal(),
            &self.return_value.to_string()
        )
    }

    fn eval(&self) -> Option<Object> {
        let obj = self.return_value.eval();

        match obj {
            None => None,
            Some(Object::Error(_)) => return obj,
            Some(obj) => Some(Object::ReturnValue(Box::new(obj))),
        }
    }
}

impl ReturnStatement {
    pub fn new(token: Token, return_value: Box<dyn Expression>) -> ReturnStatement {
        ReturnStatement {
            token,
            return_value,
        }
    }
}

pub struct BlockStatement {
    pub token: Token, // The '{' token, signifying the start of the block statement.
    pub statements: Vec<Box<dyn Statement>>,
}

impl Statement for BlockStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn statement_node(&self) {}
}

impl Node for BlockStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
    fn to_string(&self) -> String {
        format!(
            "{{ {}}}",
            self.statements
                .iter()
                .map(|stmt| format!("{}; ", stmt.to_string()))
                .collect::<String>()
                .to_string()
        )
    }

    fn eval(&self) -> Option<Object> {
        let mut res: Option<Object> = None;

        for stmt in &self.statements {
            dbg!(&stmt.to_string());
            res = stmt.eval();
            dbg!(&res);

            match &res {
                None => return None,
                Some(obj) => match obj {
                    // We are returning the original ReturnValue
                    // instead of unwrapping because if we unwrap it
                    // in a nested block, it won't serve the purpose
                    // of stopping execution in the outer scope.
                    // Once ReturnValue is reached, we need to
                    // stop execution, therefore we need to let
                    // this ReturnValue bubble up to the outermost
                    // block, where it is going to be picked up
                    // Program's eval which unwraps it. Check
                    // the Program's implementation of `eval` to
                    // see the difference.
                    Object::ReturnValue(_) => {
                        return Some(obj.clone());
                    }
                    // Same for Error object. When we encouter error, let's
                    // propagate it to the outer scope so that Program's eval
                    // can stop the execution.
                    Object::Error(_) => return Some(obj.clone()),
                    _ => continue,
                },
            }
        }

        return res;
    }
}

impl BlockStatement {
    pub fn new(token: Token, statements: Vec<Box<dyn Statement>>) -> Self {
        BlockStatement { token, statements }
    }
}

pub struct IfExpression {
    pub token: Token, // The `if` token.
    pub condition: Box<dyn Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl Expression for IfExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn expression_node(&self) {}
}

impl Node for IfExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
    fn to_string(&self) -> String {
        let mut s = String::from(format!(
            "if {} {}",
            self.condition.to_string(),
            self.consequence.to_string(),
        ));

        if self.alternative.is_some() {
            s += format!(" else {}", self.alternative.as_ref().unwrap().to_string()).as_str();
        }

        return s;
    }

    fn eval(&self) -> Option<Object> {
        let condition = self.condition.eval();

        match condition {
            None => None,
            Some(Object::Error(_)) => condition,
            Some(value) => {
                if is_truthy(value) {
                    return self.consequence.eval();
                } else if self.alternative.is_some() {
                    return self.alternative.as_ref().unwrap().eval();
                } else {
                    return Some(Object::Null);
                }
            }
        }
    }
}

impl IfExpression {
    pub fn new(
        token: Token,
        condition: Box<dyn Expression>,
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
    ) -> Self {
        IfExpression {
            token,
            condition,
            consequence,
            alternative,
        }
    }
}

pub struct FunctionLiteral {
    pub token: Token, // The `fn` token.
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

impl Expression for FunctionLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn expression_node(&self) {}
}

impl Node for FunctionLiteral {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn to_string(&self) -> String {
        format!(
            "fn({}) {}",
            &self
                .parameters
                .iter()
                .map(|param| format!("{}, ", param.to_string()))
                .collect::<String>()
                .trim_end_matches(", "),
            self.body.to_string()
        )
    }

    fn eval(&self) -> Option<Object> {
        unimplemented!()
    }
}

impl FunctionLiteral {
    pub fn new(token: Token, parameters: Vec<Identifier>, body: BlockStatement) -> Self {
        Self {
            token,
            parameters,
            body,
        }
    }
}

pub struct CallExpression {
    pub token: Token, // The `(` token signifying the start of the argument list.
    // Either a `FunctionLiteral` or an `Identifier`
    //      `FunctionLiteral`   fn(x, y) { x + y }(1, 2)
    //      `Identifier`        add(1, 2)
    pub function: Box<dyn Expression>,
    pub arguments: Vec<Box<dyn Expression>>,
}

impl Expression for CallExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn expression_node(&self) {}
}

impl Node for CallExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn to_string(&self) -> String {
        format!(
            "{}({})",
            self.function.to_string(),
            self.arguments
                .iter()
                .map(|arg| format!("{}, ", arg.to_string()))
                .collect::<String>()
                .trim_end_matches(", ")
        )
    }

    fn eval(&self) -> Option<Object> {
        unimplemented!()
    }
}

impl CallExpression {
    pub fn new(
        token: Token,
        function: Box<dyn Expression>,
        arguments: Vec<Box<dyn Expression>>,
    ) -> Self {
        Self {
            token,
            function,
            arguments,
        }
    }
}

#[derive(Debug)]
pub struct Boolean {
    pub token: Token,
    pub value: bool,
}

impl Expression for Boolean {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn expression_node(&self) {}
}

impl Node for Boolean {
    fn to_string(&self) -> String {
        self.token.literal.clone()
    }

    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn eval(&self) -> Option<Object> {
        Some(Object::Boolean(self.value))
    }
}

impl Boolean {
    pub fn new(token: Token, value: bool) -> Self {
        Boolean { token, value }
    }
}
