use crate::ast::AST;

enum ZerangValue {
    ZNil,
    ZBoolean(bool),
    ZNumber(f64),
    ZString(String),
}
pub struct Output {}
#[derive(Debug)]
pub struct Error {}

pub fn evaluate(ast: AST) -> Result<Output, Error> {
    println!("Evaluating");

    Ok(Output {})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        assert_eq!(true, true)
    }
}
