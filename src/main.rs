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
    read_source();
    tokenize();
    parse();
    evaluate();
}
