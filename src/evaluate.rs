use crate::parser::AST;

/// Run a zerang program

pub type Output = ();

pub fn evaluate(ast: AST) -> Output {
    println!("Evaluating");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        assert_eq!(true, true)
    }
}
