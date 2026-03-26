use crate::tokenize::Tokens;

pub struct AST {}
#[derive(Debug)]
pub struct Error {}

pub fn parse(tokens: Tokens) -> Result<AST, Error> {
    println!("parsing");

    Ok(AST {})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        assert_eq!(true, true)
    }
}
