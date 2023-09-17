#[cfg(test)]
mod parsers_tests {
    use monkey_interpreter::ast::{LetStatement, Node, Statement};
    use monkey_interpreter::lexer::Lexer;
    use monkey_interpreter::parser::Parser;

    #[test]
    fn should_parse_input() {
        let input = "
        let x = 5;
        let y = 10;
        let foobar = 12345;
        ";

        let lex = Lexer::new(&input.to_string());
        let mut parser = Parser::new(lex);

        let program = parser.parse_program();
        assert!(program.is_some());
        check_parse_errors(&parser);

        let program = program.unwrap();
        assert_eq!(program.statements.len(), 3);

        let tests = ["x".to_string(), "y".to_string(), "foobar".to_string()];

        for (i, name) in tests.iter().enumerate() {
            let stmt = &program.statements[i];
            assert!(test_let_statement(stmt, name));
        }
    }

    #[test]
    fn should_record_parsing_errors() {
        let input = "
        let x 5;
        let = 10;
        let 12345;
        ";

        let lex = Lexer::new(&input.to_string());
        let mut parser = Parser::new(lex);

        parser.parse_program();
        assert_eq!(parser.get_errors().len(), 3);
    }

    fn check_parse_errors(parser: &Parser) {
        let errors = parser.get_errors();
        if errors.len() == 0 {
            return;
        }

        eprintln!("encoutered {} errors during parsing", errors.len());

        for err in errors {
            eprintln!("parser error: {}", err);
        }

        panic!();
    }

    fn test_let_statement(stmt: &Box<dyn Statement>, name: &String) -> bool {
        if stmt.token_literal() != "let" {
            eprintln!(
                "statement's token literal is not 'let', got={}",
                stmt.token_literal()
            );
            return false;
        }

        if let Some(let_stmt) = stmt.as_any().downcast_ref::<LetStatement>() {
            if &let_stmt.name.value != name {
                eprintln!(
                    "let_statement name is not {}, got={}",
                    name, let_stmt.name.value
                );
                return false;
            }
            if &let_stmt.name.token_literal() != name {
                eprintln!(
                    "statement name is not {}, got={}",
                    name,
                    let_stmt.name.token_literal()
                );
                return false;
            }
        } else {
            eprintln!("statement is not LetStatement.");
            return false;
        }

        return true;
    }
}
