mod ast;
mod evaluate;
mod parser;
mod reader;
mod tokenize;

use std::{
    collections::HashMap,
    env::args,
    io::{Write, stdin, stdout},
};

use evaluate::evaluate;
use parser::parse;
use reader::read_source;
use tokenize::tokenize;

use crate::reader::Source;

#[derive(Debug)]
pub enum Error {
    Read(reader::Error),
    Tokenize(tokenize::Error),
    Parse(parser::Error),
    Evaluate(evaluate::Error),
}

impl From<reader::Error> for Error {
    fn from(err: reader::Error) -> Self {
        Error::Read(err)
    }
}

impl From<tokenize::Error> for Error {
    fn from(err: tokenize::Error) -> Self {
        Error::Tokenize(err)
    }
}

impl From<parser::Error> for Error {
    fn from(err: parser::Error) -> Self {
        Error::Parse(err)
    }
}

impl From<evaluate::Error> for Error {
    fn from(err: evaluate::Error) -> Self {
        Error::Evaluate(err)
    }
}

fn run(source: Source) -> Result<(), Error> {
    let tokens = tokenize(source)?;
    println!("{tokens:?}");
    let ast = parse(tokens)?;
    let output = evaluate(ast)?;
    Ok(())
}

fn run_file(filename: &str) -> Result<(), Error> {
    let source = read_source(filename)?;
    run(source)
}

fn run_prompt() {
    loop {
        stdout().write(b"> ").unwrap();
        stdout().flush().unwrap();
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        let source = Source { contents: buffer };
        match run(source) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("{err:?}");
            }
        }
    }
}

fn main() {
    println!("Hello, Zerang!");

    let args = args().collect::<Vec<_>>();

    if args.len() == 1 {
        run_prompt();
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => {
                println!("Success!")
            }
            Err(err) => {
                eprintln!("It failed {err:?}")
            }
        }
    } else {
        eprintln!("Usage: zerang [filename]")
    }
}
