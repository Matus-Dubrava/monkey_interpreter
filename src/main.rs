pub mod ast;
pub mod environment;
pub mod eval;
pub mod lexer;
pub mod object;
pub mod parser;
pub mod repl;
pub mod token;
pub mod utils;

use repl::start_repl;
use utils::print_parser_output_of_supported_operations;

use clap::Parser;

fn main() -> Result<(), color_eyre::Report> {
    color_eyre::install()?;
    let args = Args::parse();

    if args.supported_parsing_info > 0 {
        print_parser_output_of_supported_operations();
    }

    if args.repl > 0 {
        start_repl();
    }

    Ok(())
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, action=clap::ArgAction::Count)]
    supported_parsing_info: u8,

    #[arg(short, long, action=clap::ArgAction::Count)]
    repl: u8,
}
