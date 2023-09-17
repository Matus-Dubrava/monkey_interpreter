pub mod lexer;
pub mod token;

use lexer::lexer::Lexer;

fn main() -> Result<(), color_eyre::Report> {
    color_eyre::install()?;

    let s = String::from("=+{}()");
    let mut lex = Lexer::new(s)?;

    for _ in 0..10 {
        let tok = lex.next_token();
        println!("{:?}", lex);
        println!("{:?}", tok);
    }

    Ok(())
}
