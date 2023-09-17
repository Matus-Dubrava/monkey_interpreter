use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
}

trait Statement: Node {
    fn statement_node(&self);
}

trait Expression: Node {
    fn expression_node(&self);
}

struct Program {
    statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        } else {
            return "".to_string();
        }
    }
}

pub struct Identifier {
    token: Token,
    value: String,
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
}

pub struct LetStatement {
    token: Token,
    name: Identifier,
    value: dyn Expression,
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }
}
