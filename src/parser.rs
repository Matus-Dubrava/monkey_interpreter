use std::collections::HashMap;

use crate::ast::{
    DummyExpression, Expression, ExpressionStatement, Identifier, IntegerLiteral, LetStatement,
    Node, Program, ReturnStatement, Statement,
};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

type PrefixParseFn = fn(&mut Parser) -> Option<Box<dyn Expression>>;
type InfixParseFn = fn(&mut Parser, Box<dyn Expression>) -> Option<Box<dyn Expression>>;

// operator precendeces
const _: u8 = 0;
const LOWEST: u8 = 1;
const EQUALS: u8 = 2;
const LESSGREATER: u8 = 3;
const SUM: u8 = 4;
const PRODUCT: u8 = 5;
const PREFIX: u8 = 6;
const CALL: u8 = 7;

#[derive(Debug, Clone)]
pub struct Parser {
    lex: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
    prefix_parse_fns: HashMap<TokenType, PrefixParseFn>,
    infix_parse_fns: HashMap<TokenType, InfixParseFn>,
}

impl Parser {
    pub fn new(mut lex: Lexer) -> Self {
        let cur_token = lex.next_token();
        let peek_token = lex.next_token();
        let errors: Vec<String> = Vec::new();
        let prefix_parse_fns: HashMap<TokenType, PrefixParseFn> = HashMap::new();
        let infix_parse_fns: HashMap<TokenType, InfixParseFn> = HashMap::new();

        let mut parser = Parser {
            lex,
            cur_token,
            peek_token,
            errors,
            prefix_parse_fns,
            infix_parse_fns,
        };

        parser.register_prefix(TokenType::IDENT, Parser::parse_identifier);
        parser.register_prefix(TokenType::INT, Parser::parse_integer_literal);

        parser
    }

    pub fn register_prefix(&mut self, token_type: TokenType, fun: PrefixParseFn) {
        self.prefix_parse_fns.insert(token_type, fun);
    }

    pub fn register_infix(&mut self, token_type: TokenType, fun: InfixParseFn) {
        self.infix_parse_fns.insert(token_type, fun);
    }

    pub fn get_errors(&self) -> &Vec<String> {
        &self.errors
    }

    pub fn peek_error(&mut self, token_type: TokenType) {
        let msg = format!(
            "expected next token to be {:?}, got={:?}",
            token_type, self.peek_token.r#type
        );
        self.errors.push(msg);
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lex.next_token();
    }

    pub fn parse_program(&mut self) -> Option<Program> {
        // do we need to return an Option here?
        let mut program = Program::new();

        while !self.cur_token_is(TokenType::EOF) {
            let stmt = self.parse_statement();
            if let Some(stmt) = stmt {
                program.statements.push(stmt);
            }
            self.next_token(); // does this correctly eats semicolons?
        }

        Some(program)
    }

    pub fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.cur_token.r#type {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    pub fn parse_expression_statement(&mut self) -> Option<Box<dyn Statement>> {
        if let Some(expression) = self.parse_expression(LOWEST) {
            let token = self.cur_token.clone();

            if self.peek_token_is(TokenType::SEMICOLON) {
                self.next_token();
            }

            let expression_statement = ExpressionStatement { expression, token };
            dbg!(expression_statement.to_string());
            Some(Box::new(expression_statement))
        } else {
            None
        }
    }

    pub fn parse_expression(&mut self, precedence: u8) -> Option<Box<dyn Expression>> {
        let prefix_fn = self.prefix_parse_fns.get(&self.cur_token.r#type);
        dbg!("parsing expression...");
        dbg!(&self.cur_token);
        dbg!(prefix_fn);

        // if there is prefix function associated with current token
        // execute that function => returns left expression
        match prefix_fn {
            Some(fun) => fun(self),
            None => None,
        }

        // TODO: rest
    }

    pub fn parse_identifier(&mut self) -> Option<Box<dyn Expression>> {
        Some(Box::new(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        }))
    }

    pub fn parse_integer_literal(&mut self) -> Option<Box<dyn Expression>> {
        let value = self.cur_token.literal.parse::<i64>();

        match value.is_err() {
            true => {
                self.errors.push(format!(
                    "could not parse {} as integer",
                    self.cur_token.literal
                ));
                return None;
            }
            false => Some(Box::new(IntegerLiteral {
                token: self.cur_token.clone(),
                value: value.unwrap(),
            })),
        }
    }

    pub fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        let cur_token = self.cur_token.clone();

        self.next_token();

        // TODO: implement expression parsing
        // for now, returning DummyExpression instead
        self.skip_until_semicolon();

        Some(Box::new(ReturnStatement::new(
            cur_token,
            Box::new(DummyExpression),
        )))
    }

    pub fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        let cur_token = self.cur_token.clone();

        if !self.expect_peek_and_advance(TokenType::IDENT) {
            return None;
        }

        let identifier = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };

        if !self.expect_peek_and_advance(TokenType::ASSIGN) {
            return None;
        }

        // TODO: skipping expression until we encounter a semicolon
        // for now, we are providing a dummy expression
        self.skip_until_semicolon();

        let dummy_expression = Box::new(DummyExpression);

        Some(Box::new(LetStatement::new(
            cur_token,
            identifier,
            dummy_expression,
        )))
    }

    pub fn cur_token_is(&self, token_type: TokenType) -> bool {
        self.cur_token.r#type == token_type
    }

    pub fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.r#type == token_type
    }

    pub fn expect_peek_and_advance(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            return true;
        } else {
            self.peek_error(token_type);
            return false;
        }
    }

    pub fn skip_until_semicolon(&mut self) {
        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }
    }
}
