use std::collections::HashMap;

use crate::ast::{
    BlockStatement, Boolean, CallExpression, Expression, ExpressionStatement, FloatLiteral,
    FunctionLiteral, Identifier, IfExpression, InfixExpression, IntegerLiteral, LetStatement, Node,
    PrefixExpression, Program, ReturnStatement, Statement,
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
        parser.register_prefix(TokenType::FLOAT, Parser::parse_float_literal);
        parser.register_prefix(TokenType::LPAREN, Parser::parse_grouped_expressions);
        parser.register_prefix(TokenType::IF, Parser::parse_if_expression);
        parser.register_prefix(TokenType::FUNCTION, Parser::parse_funtion_literal);

        parser.register_infix(TokenType::PLUS, Parser::parse_infix_expression);
        parser.register_infix(TokenType::MINUS, Parser::parse_infix_expression);
        parser.register_infix(TokenType::SLASH, Parser::parse_infix_expression);
        parser.register_infix(TokenType::ASTERISK, Parser::parse_infix_expression);
        parser.register_infix(TokenType::EQ, Parser::parse_infix_expression);
        parser.register_infix(TokenType::NOTEQ, Parser::parse_infix_expression);
        parser.register_infix(TokenType::LT, Parser::parse_infix_expression);
        parser.register_infix(TokenType::GT, Parser::parse_infix_expression);
        parser.register_infix(TokenType::LPAREN, Parser::parse_call_expression);

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
        map.insert(TokenType::LPAREN, CALL);
        return map;
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

    /// Return a list of errors that were recorded during parsing.
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

    /// Parsing function associated with left parentheses (LPAREN). This function
    /// is responsible for correct parsing expressions enclosed in parentheses.
    /// i.e 10 * (5 + 5) is parsed into (10 * (5 + 5)).
    /// It works by boosting precedence of enclosed expression.
    pub fn parse_grouped_expressions(&mut self) -> Option<Box<dyn Expression>> {
        self.next_token();

        let expr = self.parse_expression(LOWEST);

        // After the enclosed expression is parsed, we expect next character
        // to be right parentheses.
        if !self.expect_peek_and_advance(TokenType::RPAREN) {
            return None;
        }

        return expr;
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
        let prefix_fn = self.prefix_parse_fns.get(&self.cur_token.r#type);
        if prefix_fn.is_none() {
            self.no_prefix_parse_fn_error(self.cur_token.r#type);
            return None;
        }

        let mut left_expr = prefix_fn.unwrap()(self);

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

    /// Parse call expression is infix expression where the significant token
    /// denoting this expression is `(`. Expression before this token is
    /// either the name of the function, resp. an `Identifier`
    ///     add(1, 2)
    /// or it is function expressions
    ///     fn(x, y) { x + y }(1, 2)
    ///
    /// In either case, this function parses only the call expression that
    /// follows `(`, rest is handled by general `parse_expression` function.
    ///
    /// Therefore, here we only need to parse the argument list itself.
    /// Note that this list can be empty.
    fn parse_call_expression(
        &mut self,
        function: Box<dyn Expression>,
    ) -> Option<Box<dyn Expression>> {
        let mut args: Vec<Box<dyn Expression>> = Vec::new();
        let cur_token = self.cur_token.clone();

        if self.peek_token_is(TokenType::RPAREN) {
            self.next_token();
            return Some(Box::new(CallExpression::new(cur_token, function, args)));
        }

        loop {
            // Advance token to start parsing arguments, or to skip current
            // `,` separating argument expressions.
            self.next_token();
            let arg = self.parse_expression(LOWEST);
            if arg.is_none() {
                self.errors.push(format!("Error while parsing call expression arguments. Failed to parse argument, current token is `{}`", &self.cur_token.literal));
                return None;
            }
            args.push(arg.unwrap());

            // Advance token, expecting either `)` signifying the end of argument
            // list, or a `,` separating arguments.
            self.next_token();

            if !self.cur_token_is(TokenType::RPAREN) && !self.cur_token_is(TokenType::COMMA) {
                self.errors.push(format!(
                    "Error while parsing call expression arguments. Expected `,` or `)`, got=`{}`",
                    &self.cur_token.literal
                ));
                return None;
            }

            // Exit loop when closing ')' is found.
            if self.cur_token_is(TokenType::RPAREN) {
                break;
            }
        }

        return Some(Box::new(CallExpression::new(cur_token, function, args)));
    }

    /// General form of `FunctionLiteral` is
    ///     fn(<parameters>) { <statements> }
    /// where
    ///     <parameters> is a `Vec<Identifier>`   
    ///     <statements> is a `BlockStatement`
    ///
    /// <parameters> can be empty, i.e. fn() {...} is allowed.
    ///
    /// This function returns None if anything fails during parsing.
    /// Note that currently the enclosing parentheses are required.
    fn parse_funtion_literal(&mut self) -> Option<Box<dyn Expression>> {
        let cur_token = self.cur_token.clone(); // The `fn` token.
        let mut parameters: Vec<Identifier> = Vec::new();

        // Advance to the next token which should be the opening `(`.
        if !self.expect_peek_and_advance(TokenType::LPAREN) {
            self.errors.push(format!(
                "Error while parsing function. Exprected next token to be `(`, got=`{}`",
                self.cur_token.literal
            ));
            return None;
        }

        // Advance to the next token. This should either be an `Identifier`
        // if there are any parameters, or it should be closing `)` if
        // this function doesn't take any parameters.
        self.next_token();

        if !self.cur_token_is(TokenType::RPAREN) {
            // Start parsing function parameters.
            // Until the closing `)` is reached, the pattern should be
            // `Identifier` followed by `,` except for the last
            // `Identifier` that is followed by the closing `)`.
            loop {
                if !self.cur_token_is(TokenType::IDENT) {
                    self.errors.push(format!("Error while parsing function parameters. Expected next token to be `Identifier`, got=`{}`", self.cur_token.literal));
                    return None;
                }

                let ident = Identifier::new(self.cur_token.clone(), self.cur_token.literal.clone());
                parameters.push(ident);
                self.next_token();

                if !self.cur_token_is(TokenType::COMMA) && !self.cur_token_is(TokenType::RPAREN) {
                    self.errors.push(format!("Erorr while parsing function parameters. Expected next token to be `,` or `)`, got=`{}`", self.peek_token.literal));
                    return None;
                }

                // If we reach comma here, skip it and continue parsing parameters.
                if self.cur_token_is(TokenType::COMMA) {
                    self.next_token();
                }

                // If we reach the closing `)` here, it means we're done parsing parameters.
                if self.cur_token_is(TokenType::RPAREN) {
                    break;
                }
            }
        }

        // Advance to the next token which should be the start of
        // function's body, resp. a `BlockStatement`.
        self.next_token();

        // If `parse_block_statements` returns None, it means that it
        // failed and we need to return None from this function as well.
        let body = self.parse_block_statement();
        if body.is_none() {
            return None;
        }

        Some(Box::new(FunctionLiteral::new(
            cur_token,
            parameters,
            body.unwrap(),
        )))
    }

    // Parse `if else` expression, `else` is optional.
    // `if else` is treated as expression which means that it evaluates
    //
    // Currently supported form of `if` expression is:
    //      if (expr) { stmts... } else { stmts... };
    //
    // to value of the last expression found in the executed block statement.
    // Currently, braces around condition `if (x == y) ...` are required,
    // this can be changed in this function.
    // TODO: implement support for `else if`, resp. support for multiple
    // alternatives.
    pub fn parse_if_expression(&mut self) -> Option<Box<dyn Expression>> {
        let cur_token = self.cur_token.clone();

        // advance to the next token which should be `(`
        // signifying start of the condition
        if !self.expect_peek_and_advance(TokenType::LPAREN) {
            self.errors.push("missing `(` after `if`".to_string());
            return None;
        }

        // advance to next token which should be the actual start of condition
        self.next_token();
        let condition = self.parse_expression(LOWEST);
        if condition.is_none() {
            self.errors.push("missing `if`'s condition".to_string());
            return None;
        }

        // advance to the next token which should be `)`
        // signifying end of the condition
        if !self.expect_peek_and_advance(TokenType::RPAREN) {
            self.errors
                .push("missing closing `)` in `if`'s condition".to_string());
            return None;
        }

        // advance to the next token which should be `{`
        // signifying start of the `consequence` block statement
        if !self.expect_peek_and_advance(TokenType::LBRACE) {
            self.errors
                .push("missing `{` after `if`'s condition".to_string());
            return None;
        }

        let consequence = self.parse_block_statement();
        // empty block statements are not allowed
        if condition.is_none() {
            self.errors.push("`if`'s consequence block".to_string());
            return None;
        }

        // check whether there is optional `else` following `consequence`
        if !self.peek_token_is(TokenType::ELSE) {
            return Some(Box::new(IfExpression::new(
                cur_token,
                condition.unwrap(),
                consequence.unwrap(),
                None,
            )));
        } else {
            // next token is `else` advance to it
            self.next_token();

            // next token following `else` should be `{`
            // signifying start of the `alternative` block statement
            if !self.expect_peek_and_advance(TokenType::LBRACE) {
                self.errors
                    .push("missing `{` after `else` denoting start of block statement".to_string());
                return None;
            }

            let alternative = self.parse_block_statement();

            if alternative.is_none() {
                self.errors.push("`if`'s consequence block".to_string());
                return None;
            }

            return Some(Box::new(IfExpression::new(
                cur_token,
                condition.unwrap(),
                consequence.unwrap(),
                alternative,
            )));
        }
    }

    /// `BlockStatement` represents a collection of statements.
    /// This functions returns `None` if any of these statements
    /// is invalid, resp. if parser failed to parse any of them.
    pub fn parse_block_statement(&mut self) -> Option<BlockStatement> {
        let cur_token = self.cur_token.clone(); // The opening `{` token.
        let mut block_statements: Vec<Box<dyn Statement>> = Vec::new();

        // Advance to the next token after `{` to start parsing statements.
        self.next_token();

        // Note on the commented line. Don't think that we should allow
        // this loop to reach EOF without failing, we expect the closing `}`.

        // while !self.cur_token_is(TokenType::RBRACE) && !self.cur_token_is(TokenType::EOF)
        while !self.cur_token_is(TokenType::RBRACE) {
            let stmt = self.parse_statement();

            if stmt.is_some() {
                block_statements.push(stmt.unwrap());
            } else {
                self.errors.push(format!(
                    "Error while parsing `BlockStatement`. Expected `Statement`, got=`{}`",
                    self.cur_token.literal
                ));
                return None;
            }

            // Go to the next token which should either be a start of another
            // statement, or a closing `}`.
            self.next_token();
        }

        Some(BlockStatement::new(cur_token, block_statements))
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
                "could not parse `{}` into integer",
                self.cur_token.literal
            ));
            return None;
        } else {
            return Some(Box::new(IntegerLiteral::new(
                self.cur_token.clone(),
                value.unwrap(),
            )));
        }
    }

    pub fn parse_float_literal(&mut self) -> Option<Box<dyn Expression>> {
        let value = self.cur_token.literal.parse::<f64>();

        if value.is_err() {
            self.errors.push(format!(
                "could not parse `{}` into float",
                self.cur_token.literal
            ));
            return None;
        } else {
            return Some(Box::new(FloatLiteral::new(
                self.cur_token.clone(),
                value.unwrap(),
            )));
        }
    }

    pub fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        let cur_token = self.cur_token.clone(); // The `RETURN` token.

        // Advance to the expression after `return` token;
        self.next_token();

        let expr = self.parse_expression(LOWEST);
        if expr.is_none() {
            return None;
        }

        // Skip the `;` if there is one after return statement.
        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(Box::new(ReturnStatement::new(cur_token, expr.unwrap())))
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

        // Advance to the next token to start parsing expression that
        // follows assignment.
        self.next_token();

        let expr = self.parse_expression(LOWEST);
        if expr.is_none() {
            return None;
        }

        // Skip the semicolon if there is one after LetStatement.
        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(Box::new(LetStatement::new(
            cur_token,
            identifier,
            expr.unwrap(),
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

    /// Skip parsing of current statement, resp. until a `;` or EOF is
    /// reached.
    /// Note that this is a helper function that is not really used in
    /// parsing out the real program. Its helps in cases when
    /// implementation of certain feature is missing and we want to
    /// skip to the end of current statement.
    pub fn skip_until_semicolon(&mut self) {
        while !self.cur_token_is(TokenType::SEMICOLON) && !self.cur_token_is(TokenType::EOF) {
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
