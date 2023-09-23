use std::collections::HashMap;
use std::str::FromStr;

use crate::ast::{
    Boolean, DummyExpression, Expression, ExpressionStatement, Identifier, InfixExpression,
    IntegerLiteral, LetStatement, Node, PrefixExpression, Program, ReturnStatement, Statement,
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
    precedences: HashMap<TokenType, u8>,
}

impl Parser {
    pub fn new(mut lex: Lexer) -> Self {
        let cur_token = lex.next_token();
        let peek_token = lex.next_token();
        let errors: Vec<String> = Vec::new();
        let prefix_parse_fns: HashMap<TokenType, PrefixParseFn> = HashMap::new();
        let infix_parse_fns: HashMap<TokenType, InfixParseFn> = HashMap::new();
        let precedences = Parser::initialize_precedences();

        let mut parser = Parser {
            lex,
            cur_token,
            peek_token,
            errors,
            prefix_parse_fns,
            infix_parse_fns,
            precedences,
        };

        parser.register_prefix(TokenType::IDENT, Parser::parse_identifier);
        parser.register_prefix(TokenType::INT, Parser::parse_integer_literal);
        parser.register_prefix(TokenType::BANG, Parser::parse_prefix_expression);
        parser.register_prefix(TokenType::MINUS, Parser::parse_prefix_expression);
        parser.register_prefix(TokenType::TRUE, Parser::parse_boolean);
        parser.register_prefix(TokenType::FALSE, Parser::parse_boolean);

        parser.register_infix(TokenType::PLUS, Parser::parse_infix_expression);
        parser.register_infix(TokenType::MINUS, Parser::parse_infix_expression);
        parser.register_infix(TokenType::SLASH, Parser::parse_infix_expression);
        parser.register_infix(TokenType::ASTERISK, Parser::parse_infix_expression);
        parser.register_infix(TokenType::EQ, Parser::parse_infix_expression);
        parser.register_infix(TokenType::NOTEQ, Parser::parse_infix_expression);
        parser.register_infix(TokenType::LT, Parser::parse_infix_expression);
        parser.register_infix(TokenType::GT, Parser::parse_infix_expression);

        parser
    }

    pub fn from_str(input: &str) -> Self {
        let lex = Lexer::new(&input.to_string());
        return Parser::new(lex);
    }

    pub fn initialize_precedences() -> HashMap<TokenType, u8> {
        let mut map: HashMap<TokenType, u8> = HashMap::new();
        map.insert(TokenType::EQ, EQUALS);
        map.insert(TokenType::NOTEQ, EQUALS);
        map.insert(TokenType::LT, LESSGREATER);
        map.insert(TokenType::GT, LESSGREATER);
        map.insert(TokenType::PLUS, SUM);
        map.insert(TokenType::MINUS, SUM);
        map.insert(TokenType::SLASH, PRODUCT);
        map.insert(TokenType::ASTERISK, PRODUCT);

        map
    }

    /// returns precedence of next token
    pub fn peek_precedence(&self) -> u8 {
        if let Some(prec) = self.precedences.get(&self.peek_token.r#type) {
            *prec
        } else {
            LOWEST
        }
    }

    pub fn current_precedence(&self) -> u8 {
        if let Some(prec) = self.precedences.get(&self.cur_token.r#type) {
            *prec
        } else {
            LOWEST
        }
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

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        while !self.cur_token_is(TokenType::EOF) {
            let stmt = self.parse_statement();
            if let Some(stmt) = stmt {
                program.statements.push(stmt);
            }
            self.next_token(); // does this correctly eats semicolons?
        }

        return program;
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

            let expression_statement = ExpressionStatement::new(token, expression);
            Some(Box::new(expression_statement))
        } else {
            None
        }
    }

    pub fn parse_expression(&mut self, precedence: u8) -> Option<Box<dyn Expression>> {
        dbg!(
            "[PARSE_EXPRESSION] cur_token: {}",
            self.cur_token.r#type.to_string()
        );

        let prefix_fn = self.prefix_parse_fns.get(&self.cur_token.r#type);
        if prefix_fn.is_none() {
            dbg!(
                "\t[PARSE_EXPRESSION] prefix parse function for token {}, NOT found",
                self.cur_token.r#type.to_string()
            );
            return None;
        } else {
            dbg!(
                "\t[PARSE_EXPRESSION] prefix parse function for token {}, found",
                self.cur_token.r#type.to_string()
            );
        }

        let mut left_expr = prefix_fn.unwrap()(self);
        dbg!(
            "\t[PARSE_EXPRESSION] letf_expr: {}",
            left_expr.as_ref().unwrap().to_string()
        );

        while !self.peek_token_is(TokenType::SEMICOLON) && precedence < self.peek_precedence() {
            // do we need to clone this or is there a better way to resolve this
            // issue with borrowing
            let infix_parse_fns = self.infix_parse_fns.clone();
            if let Some(infix_fn) = infix_parse_fns.get(&self.peek_token.r#type) {
                self.next_token();

                left_expr = infix_fn(self, left_expr.unwrap());
            } else {
                return left_expr;
            }
        }
        dbg!(
            "\t[PARSE_EXPRESSION] parsed expression: {}",
            left_expr.as_ref().unwrap().to_string()
        );

        return left_expr;
    }

    pub fn parse_prefix_expression(&mut self) -> Option<Box<dyn Expression>> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();

        self.next_token();

        let right = self.parse_expression(PREFIX);

        if let Some(right) = right {
            Some(Box::new(PrefixExpression::new(
                token,
                operator.as_str(),
                right,
            )))
        } else {
            None
        }
    }

    pub fn parse_infix_expression(
        &mut self,
        left: Box<dyn Expression>,
    ) -> Option<Box<dyn Expression>> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();
        let precedence = self.current_precedence();

        self.next_token();

        let right = self.parse_expression(precedence);

        if let Some(right) = right {
            Some(Box::new(InfixExpression::new(
                token,
                left,
                operator.as_str(),
                right,
            )))
        } else {
            None
        }
    }

    pub fn parse_boolean(&mut self) -> Option<Box<dyn Expression>> {
        Some(Box::new(Boolean::new(
            self.cur_token.clone(),
            self.cur_token_is(TokenType::TRUE),
        )))
    }

    pub fn parse_identifier(&mut self) -> Option<Box<dyn Expression>> {
        Some(Box::new(Identifier::new(
            self.cur_token.clone(),
            self.cur_token.literal.clone(),
        )))
    }

    pub fn parse_integer_literal(&mut self) -> Option<Box<dyn Expression>> {
        let value = self.cur_token.literal.parse::<i64>();

        if value.is_err() {
            self.errors.push(format!(
                "could not parse {} as integer",
                self.cur_token.literal
            ));
            return None;
        } else {
            Some(Box::new(IntegerLiteral::new(
                self.cur_token.clone(),
                value.unwrap(),
            )))
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

    pub fn no_prefix_parse_fn_error(&mut self, token_type: TokenType) {
        let msg = format!(
            "no prefix parse function found for `{}`",
            token_type.to_string()
        );
        self.errors.push(msg);
    }
}
