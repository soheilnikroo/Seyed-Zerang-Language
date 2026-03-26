use crate::reader::Source;

#[derive(Debug)]
pub enum TokenType {
    // Single character token
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,
    // One or two character
    Bang,
    Bang_Equal,
    Equal,
    EqualEqual,
    Greater,
    GreatEqual,
    Less,
    LessEqual,
    // Literal
    Identifier,
    String,
    Number,
    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    Var,
    While,
    Eof,
}

#[derive(Debug)]
pub enum Literal {
    Str(String),
    Num(f64),
    None,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: usize,
}

impl Token {
    fn new(token_type: TokenType, lexeme: &str, literal: Literal, line: usize) -> Self {
        Self {
            token_type,
            lexeme: String::from(lexeme),
            literal,
            line,
        }
    }
}

#[derive(Debug)]
pub struct Tokens {
    pub tokens: Vec<Token>,
}

#[derive(Debug)]
pub struct Error {}

struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_tokens(mut self) -> Tokens {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens
            .push(Token::new(TokenType::Eof, "", Literal::None, self.line));
        Tokens {
            tokens: self.tokens,
        }
    }
    fn scan_token(&mut self) {
        todo!()
    }
}

pub fn tokenize(source: Source) -> Result<Tokens, Error> {
    println!("Tokenizing");
    let tokens = vec![];
    Ok(Tokens { tokens: tokens })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        assert_eq!(true, true)
    }
}
