use crate::token::Token;
use std::any::Any;

pub trait Node {
    fn token_literal(&self) -> &str;
    fn to_string(&self) -> String;
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
        let mut buf = String::new();

        for stmt in &self.statements {
            buf += &stmt.to_string();
        }

        buf
    }
}

impl Program {
    pub fn new() -> Self {
        let statements: Vec<Box<dyn Statement>> = Vec::new();

        Program { statements }
    }
}

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
}

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
        let mut buf = String::new();

        buf += self.token_literal();
        buf += " ";
        buf += &self.name.to_string();
        buf += " = ";
        buf += &self.value.to_string();
        buf += ";";

        buf
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
        let mut buf = String::new();

        buf += self.token_literal();
        buf += " ";
        buf += &self.return_value.to_string();
        buf += ";";

        buf
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
