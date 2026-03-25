mod evaluate;
mod parser;
mod reader;
mod tokenize;

use evaluate::evaluate;
use parser::parse;
use reader::read_source;
use tokenize::tokenize;

type Error = ();

fn run() -> Result<(), Error> {
    let source = read_source("some_file.zerang")?;
    let tokens = tokenize(source)?;
    let ast = parse(tokens)?;
    let output = evaluate(ast)?;
    Ok(())
}

fn main() {
    println!("Hello, Zerang!");
    match run() {
        Ok(_) => {
            println!("Success!")
        }
        Err(err) => {
            eprintln!("It failed {err:?}")
        }
    }
}
