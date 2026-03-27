use std::usize;

use crate::{
    ast::{AST, Expr, Operator},
    tokenize::{Token, TokenType, Tokens},
};

use Operator::*;
use TokenType::*;

impl From<&Token> for Operator {
    fn from(token: &Token) -> Self {
        match token.token_type {
            TPlus => OAdd,
            TMinus => OSub,
            TStar => OMul,
            TSlash => ODiv,
            TLess => OLt,
            TLessEqual => OLe,
            TGreater => OGt,
            TGreaterEqual => OGe,
            TEqualEqual => OEq,
            TBangEqual => ONe,
            TAnd => OAnd,
            TOr => OOr,
            TBang => ONot,
            _ => panic!("Not an operator {:?}", token.token_type),
        }
    }
}

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

    fn accepts<const N: usize>(&mut self, token_type: [TokenType; N]) -> bool {
        if !self.at_end() && token_type.contains(&self.tokens[self.n].token_type) {
            self.n += 1;
            true
        } else {
            false
        }
    }

    fn last_token(&self) -> &Token {
        &self.tokens[self.n - 1]
    }

    fn last_lexeme(&self) -> &String {
        &self.tokens[self.n - 1].lexeme
    }

    fn at_end(&self) -> bool {
        self.n >= self.tokens.len()
    }

    fn parse_expression(&mut self) -> Expr {
        let left = self.parse_term();

        if self.accepts([TPlus, TMinus, TStar, TSlash]) {
            let operator = Operator::from(self.last_token());
            let right = self.parse_term();
            Expr::binary(left, operator, right)
        } else {
            left
        }
    }

    fn parse_term(&mut self) -> Expr {
        if self.accept(TNumber) {
            Expr::number(self.last_lexeme())
        } else if self.accept(TString) {
            Expr::string(self.last_lexeme())
        } else {
            panic!("Syntax Error")
        }
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
