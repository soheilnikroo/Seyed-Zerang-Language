mod evaluate;
mod parser;
mod reader;
mod tokenize;

use evaluate::evaluate;
use parser::parse;
use reader::read_source;
use tokenize::tokenize;

fn main() {
    println!("Hello, Zerang!");
    let source = read_source("some_file.zerang");
    let tokens = tokenize(source);
    let ast = parse(tokens);
    let output = evaluate(ast);
    println!("{output:?}");
}
