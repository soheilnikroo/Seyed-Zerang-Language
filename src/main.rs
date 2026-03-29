mod ast;
mod environment;
mod evaluate;
mod parser;
mod reader;
mod tokenize;

use std::{
    env::args,
    io::{Write, stdin, stdout},
};

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

fn report_errors(err: Error) {
    match err {
        Error::Read(e) => {
            eprintln!("{}", e.msg);
        }
        Error::Tokenize(e) => {
            use crate::tokenize::ScanError;
            for scan_error in e.iter() {
                match scan_error {
                    ScanError::UnexpectedCharacter { line, ch } => {
                        eprintln!("Line {line}: Unexpected character {ch:?}");
                    }
                    ScanError::UnterminatedString { line } => {
                        eprintln!("Line {line}: Unterminated string");
                    }
                }
            }
        }
        Error::Parse(e) => {
            use crate::parser::Error;
            match e {
                Error::SyntaxError { line, msg } => {
                    eprintln!("Line {line}: Syntax error: {msg}");
                }
            }
        }
        Error::Evaluate(e) => {
            use crate::evaluate::Error::*;
            match e {
                ZeroDivision => {
                    eprintln!("Division by zero");
                }
                UnsupportedBinOp(left, op, right) => {
                    eprintln!("Unsupported operation: {left:?} {op} {right:?}");
                }
                UnsupportedUnaryOp(op, value) => {
                    eprintln!("Unsupported operation: {op}{value:?}");
                }
                NotFound(name) => {
                    eprintln!("{name} not found");
                }
            }
        }
    }
}

fn run(source: Source) -> Result<(), Error> {
    let mut interpreter = evaluate::Interpreter::new();
    run_interpreter(&mut interpreter, source)
}

fn run_interpreter(interpreter: &mut evaluate::Interpreter, source: Source) -> Result<(), Error> {
    let tokens = tokenize(&source)?;
    let ast = parse(tokens)?;
    interpreter.evaluate(ast)?;

    Ok(())
}

fn run_file(filename: &str) -> Result<(), Error> {
    let source = read_source(filename)?;
    run(source)
}

fn run_prompt() {
    let mut interpreter = evaluate::Interpreter::new();
    loop {
        stdout().write(b"> ").unwrap();
        stdout().flush().unwrap();
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        let source = Source { contents: buffer };
        match run_interpreter(&mut interpreter, source) {
            Ok(_) => {}
            Err(err) => {
                report_errors(err);
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
                report_errors(err);
            }
        }
    } else {
        eprintln!("Usage: zerang [filename]")
    }
}
