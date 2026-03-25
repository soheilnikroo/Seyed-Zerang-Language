use crate::parser::AST;

/// Run a zerang program

pub type Output = ();

pub fn evaluate(ast: AST) -> Output {
    println!("Evaluating");
}
