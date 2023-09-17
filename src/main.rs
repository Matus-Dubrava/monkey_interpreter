pub mod ast;
pub mod lexer;
pub mod parser;
pub mod repl;
pub mod token;

use repl::start_repl;

fn main() -> Result<(), color_eyre::Report> {
    color_eyre::install()?;

    start_repl();

    Ok(())
}
