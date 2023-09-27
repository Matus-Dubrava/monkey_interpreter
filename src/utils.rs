use crate::lexer::Lexer;
use crate::{ast::Node, parser::Parser};

/// Parses input into program and returns its
/// string represenation.
pub fn get_stringified_parser_output(input: &str) -> String {
    let mut parser = Parser::from_str(input);
    let program = parser.parse_program();
    format!("{}", program.to_string())
}

/// Returns string represetation of lexer's output.
pub fn get_stringified_lexer_output(input: &str) -> String {
    let mut lex = Lexer::new(&input.to_string());
    let tokens = lex.get_all_tokens();
    format!(
        "{}",
        tokens
            .iter()
            .map(|tok| format!("{}, ", tok.to_string()))
            .collect::<String>()
            .trim_end_matches(", ")
    )
}

/// Prints out both parser and lexer stringified outputs.
pub fn print_parser_output(input: &str) {
    let program_str = get_stringified_parser_output(&input);
    let tokens_str = get_stringified_lexer_output(&input);

    println!("PARSER: {program_str}");
    println!("TOKENIZER: {tokens_str}");
}

pub fn print_parser_output_of_supported_operations() {
    let functions = vec![
        "fn () {}",
        "fn () { x }",
        "fn (x) {}",
        "fn (x, y) { x + 1 }",
        "let add = (x, y) { x + y }",
    ];

    for function in functions {
        println!("INPUT: {}", function);
        print_parser_output(function);
        println!();
    }

    let if_expressions = vec!["if (x) {}", "if (true) { x + 1 } else {x - 1}"];

    for expr in if_expressions {
        println!("INPUT: {}", expr);
        print_parser_output(expr);
        println!();
    }
}
