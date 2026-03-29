use std::usize;

use crate::{
    ast::{AST, Expr, Operator, Statement},
    tokenize::{Token, TokenType, Tokens},
};

use Operator::*;
use TokenType::*;

impl<'a> From<&Token<'a>> for Operator {
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
pub enum Error {
    SyntaxError { line: usize, msg: String },
}

struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    n: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: Tokens<'a>) -> Self {
        Self {
            tokens: tokens.tokens,
            n: 0,
        }
    }

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

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<(), Error> {
        if !self.accept(token_type) {
            Err(self.syntax_error(msg))
        } else {
            Ok(())
        }
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        !self.at_end() && self.tokens[self.n].token_type == token_type
    }

    fn syntax_error(&self, msg: &str) -> Error {
        Error::SyntaxError {
            line: self.tokens[self.n].line,
            msg: format!("{msg} at {:?}", self.tokens[self.n].lexeme),
        }
    }

    fn last_token(&self) -> &Token<'_> {
        &self.tokens[self.n - 1]
    }

    fn last_lexeme(&self) -> &&'a str {
        &self.tokens[self.n - 1].lexeme
    }

    fn at_end(&self) -> bool {
        self.n >= self.tokens.len() || self.tokens[self.n].token_type == TEof
    }

    fn parse_top(&mut self) -> Result<AST, Error> {
        let top = self.parse_statements()?;
        if !self.at_end() {
            return Err(self.syntax_error("Unparsed input"));
        }
        Ok(AST { top })
    }

    fn parse_statements(&mut self) -> Result<Vec<Statement>, Error> {
        let mut statements = Vec::new();
        while !self.at_end() {
            statements.push(self.parse_declaration()?);
        }
        Ok(statements)
    }

    fn parse_declaration(&mut self) -> Result<Statement, Error> {
        if self.accept(TVar) {
            self.parse_var_declaration()
        } else {
            self.parse_statement()
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, Error> {
        if self.accept(TPrint) {
            self.parse_print_statement()
        } else if self.accept(TLeftBrace) {
            self.parse_block()
        } else if self.accept(TIf) {
            self.parse_if_statement()
        } else if self.accept(TWhile) {
            self.parse_while_statement()
        } else {
            self.parse_expression_statement()
        }
    }

    fn parse_block(&mut self) -> Result<Statement, Error> {
        let mut statements = Vec::new();
        while !self.check(TRightBrace) && !self.at_end() {
            statements.push(self.parse_declaration()?);
        }
        self.consume(TRightBrace, "Expect '}' after block.")?;
        Ok(Statement::block(statements))
    }

    fn parse_print_statement(&mut self) -> Result<Statement, Error> {
        let value = self.parse_expression()?;
        self.consume(TSemiColon, "Expect ';' after value.")?;
        Ok(Statement::print(value))
    }

    fn parse_if_statement(&mut self) -> Result<Statement, Error> {
        self.consume(TLeftParen, "Expect '(' after if.")?;
        let condition = self.parse_expression()?;
        self.consume(TRightParen, "Expected ')' after if condition.")?;
        let consequence = self.parse_statement()?;
        let mut alternative = None;
        if self.accept(TElse) {
            alternative = Some(self.parse_statement()?);
        }
        Ok(Statement::if_statement(condition, consequence, alternative))
    }

    fn parse_while_statement(&mut self) -> Result<Statement, Error> {
        self.consume(TLeftParen, "Expect '(' after 'while'.")?;
        let condition = self.parse_expression()?;
        self.consume(TRightParen, "Expect ')' after 'while' condition.")?;
        let body = self.parse_statement()?;
        Ok(Statement::while_statement(condition, body))
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, Error> {
        let value = self.parse_expression()?;
        self.consume(TSemiColon, "Expect ';' after value.")?;
        Ok(Statement::expression(value))
    }

    fn parse_var_declaration(&mut self) -> Result<Statement, Error> {
        self.consume(TIdentifier, "Expect variable name")?;
        let name = *self.last_lexeme();
        let mut initializer = None;
        if self.accept(TEqual) {
            initializer = Some(self.parse_expression()?);
        }
        self.consume(TSemiColon, "Expect ';' after variable declaration.")?;
        Ok(Statement::var_decl(name, initializer))
    }

    fn parse_expression(&mut self) -> Result<Expr, Error> {
        self.parse_assignment()
    }

    pub fn parse_assignment(&mut self) -> Result<Expr, Error> {
        let expr = self.parse_binary()?;
        if self.accept(TEqual) {
            let value = self.parse_assignment()?;
            if let Expr::EVariable { name } = expr {
                return Ok(Expr::assign(name, value));
            } else {
                panic!("Invalid assignment target");
            }
        }
        Ok(expr)
    }

    pub fn parse_binary(&mut self) -> Result<Expr, Error> {
        let left = self.parse_unary()?;
        if self.accepts([
            TPlus,
            TMinus,
            TStar,
            TSlash,
            TLess,
            TLessEqual,
            TGreater,
            TGreaterEqual,
            TEqualEqual,
            TBangEqual,
        ]) {
            let op = Operator::from(self.last_token());
            let right = self.parse_unary()?;
            Ok(Expr::binary(left, op, right))
        } else {
            Ok(left)
        }
    }

    fn parse_unary(&mut self) -> Result<Expr, Error> {
        if self.accepts([TMinus, TBang]) {
            let operator = Operator::from(self.last_token());
            Ok(Expr::unary(operator, self.parse_unary()?))
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, Error> {
        Ok(if self.accept(TNumber) {
            Expr::number(self.last_lexeme().to_string())
        } else if self.accept(TString) {
            let lexeme = self.last_lexeme();
            Expr::string(&lexeme[1..lexeme.len() - 1])
        } else if self.accept(TNil) {
            Expr::nil()
        } else if self.accept(TTrue) {
            Expr::bool(true)
        } else if self.accept(TFalse) {
            Expr::bool(false)
        } else if self.accept(TLeftParen) {
            let expr = self.parse_expression()?;
            self.consume(TRightParen, "Expected ')' after expression")?;
            Expr::grouping(expr)
        } else if self.accept(TIdentifier) {
            Expr::variable(self.last_lexeme().to_string())
        } else {
            return Err(self.syntax_error("Expected primary"));
        })
    }
}

pub fn parse(tokens: Tokens) -> Result<AST, Error> {
    Parser::new(tokens).parse_top()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_expr_string(source: &str) -> Expr {
        use crate::reader::Source;
        use crate::tokenize::tokenize;

        let source = Source::from(source);
        let tokens = tokenize(&source).unwrap();
        Parser::new(tokens).parse_expression().unwrap()
    }

    #[test]
    fn its_alive() {
        assert_eq!(true, true)
    }

    #[test]
    fn test_primary() {
        assert_eq!(parse_expr_string("123"), Expr::number("123"));

        assert_eq!(parse_expr_string("\"hello\""), Expr::string("hello"));

        assert_eq!(parse_expr_string("nil"), Expr::nil());

        assert_eq!(parse_expr_string("true"), Expr::bool(true));

        assert_eq!(parse_expr_string("false"), Expr::bool(false));

        assert_eq!(parse_expr_string("(2)"), Expr::grouping(Expr::number("2")));
    }

    #[test]
    fn test_binary() {
        assert_eq!(
            parse_expr_string("1 + 2"),
            Expr::binary(Expr::number("1"), OAdd, Expr::number("2"))
        )
    }
}
