use crate::tokenize::Tokens;

pub type AST = ();
pub type Error = ();

pub fn parse(tokens: Tokens) -> Result<AST, Error> {
    println!("parsing");

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
