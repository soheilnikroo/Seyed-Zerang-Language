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
    let source = read_source("some_file.zerang").unwrap();
    let tokens = tokenize(source).unwrap();
    let ast = parse(tokens).unwrap();
    let output = evaluate(ast).unwrap();
}
