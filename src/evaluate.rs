use crate::parser::AST;

/// Run a zerang program

pub type Output = ();
pub type Error = ();

pub fn evaluate(ast: AST) -> Result<Output, Error> {
    println!("Evaluating");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        assert_eq!(true, true)
    }
}
