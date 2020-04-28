mod ast;
mod builtins;
mod code;
mod compiler;
mod environment;
mod error;
mod eval;
mod lexer;
mod object;
mod parser;
mod repl;
mod token;
mod vm;
mod test_utils;

use error::MonkeyError;
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<(), MonkeyError> {
    let mut args = std::env::args();
    if let Some(path) = args.nth(1) {
        if let Err(e) = run_program_file(path) {
            eprintln!("{}", e);
        }
    } else {
        repl::start()?;
    }
    Ok(())
}

fn run_program_file(path: String) -> Result<(), MonkeyError> {
    let reader = BufReader::new(File::open(path)?);
    let lexer = lexer::Lexer::new(Box::new(reader))?;
    let parsed_program = parser::Parser::new(lexer)?.parse_program()?;
    eval::run_program(parsed_program)?;
    Ok(())
}
