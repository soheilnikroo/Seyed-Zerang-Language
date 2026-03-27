use crate::{
    ast::AST,
    tokenize::{Token, TokenType, Tokens},
};

#[derive(Debug)]
pub struct Error {}

struct Parser {
    tokens: Vec<Token>,
    n: usize,
}

impl Parser {
    fn accept(&mut self, token_type: TokenType) -> bool {
        if !self.at_end() && self.tokens[self.n].token_type == token_type {
            self.n += 1;
            true
        } else {
            false
        }
    }

    fn last(&self) -> &Token {
        &self.tokens[self.n - 1]
    }

    fn at_end(&self) -> bool {
        self.n >= self.tokens.len()
    }
}

pub fn parse(tokens: Tokens) -> Result<AST, Error> {
    println!("parsing");
    Ok(AST { top: None })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        assert_eq!(true, true)
    }
}
