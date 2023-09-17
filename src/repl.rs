use std::io::Write;

use crate::lexer::lexer::Lexer;
use crate::token::{Token, TokenType};

pub fn start_repl() {
    let mut input = String::new();

    loop {
        print!(">> ");
        std::io::stdout().flush().expect("failed to flush stdout");

        match std::io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {
                let mut lex = Lexer::new(&input);
                let mut tokens: Vec<Token> = Vec::new();

                loop {
                    // add null character so that lexer knows when to stop
                    input += &'\0'.to_string();

                    let tok = lex.next_token();
                    tokens.push(tok.clone());

                    if tok.r#type == TokenType::EOF {
                        break;
                    }
                }

                // print all of the tokens except for the last one
                // that was artifically added to the user input
                // so that the lexer knows when to stop reading
                for (index, tok) in tokens.iter().enumerate() {
                    if index < tokens.len() - 1 {
                        println!("{tok:?}");
                    }
                }
            }
            Err(err) => println!("error: {err}"),
        }
        input.clear();
    }
}
