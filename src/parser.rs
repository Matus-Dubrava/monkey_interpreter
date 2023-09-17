use crate::ast::{DummyExpression, Identifier, LetStatement, Program, Statement};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

#[derive(Debug, Clone)]
pub struct Parser {
    lex: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(mut lex: Lexer) -> Self {
        let cur_token = lex.next_token();
        let peek_token = lex.next_token();

        Parser {
            lex,
            cur_token,
            peek_token,
        }
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lex.next_token();
    }

    pub fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program::new();

        while !self.cur_token_is(TokenType::EOF) {
            let stmt = self.parse_statement();
            if let Some(stmt) = stmt {
                program.statements.push(stmt);
            }
        }

        Some(program)
    }

    pub fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.cur_token.r#type {
            TokenType::LET => self.parse_let_statement(),
            // TODO: we need better handling of error here
            // than just panicking, we need to decide whether we want to
            // return None here, it can possibly conflict with None
            // that is already being returned from the above parse methods
            _ => panic!("parse statement error"),
        }
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
        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }
        // eat the semicolon as well
        self.next_token();

        let dummy_expression = Box::new(DummyExpression);

        Some(Box::new(LetStatement {
            name: identifier,
            token: cur_token,
            value: dummy_expression,
        }))
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
            return false;
        }
    }
}
