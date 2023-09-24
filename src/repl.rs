use std::io::Write;

use crate::ast::Node;
use crate::parser::Parser;

pub fn start_repl() {
    let mut input = String::new();

    loop {
        // It is essential to clear the input buffer here, otherwise the
        // input will just keep accumulating and if there are any errors,
        // all subsequent requests will fail.
        input.clear();
        print!(">> ");
        std::io::stdout().flush().expect("failed to flush stdout");

        match std::io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {
                let mut parser = Parser::from_str(&input);
                let program = parser.parse_program();

                // Check for parsing errors, print them if there are any.
                if parser.get_errors().len() != 0 {
                    for err in parser.get_errors() {
                        println!("{err}");
                    }
                    continue;
                }

                println!("{}", program.to_string());
            }
            Err(err) => println!("error: {err}"),
        }
        input.clear();
    }
}
