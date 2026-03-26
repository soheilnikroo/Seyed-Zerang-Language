use crate::reader::Source;

#[derive(Debug, PartialEq)]
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
    BangEqual,
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

#[derive(Debug, PartialEq)]
pub enum Literal {
    Str(String),
    Num(f64),
    None,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: usize,
}

impl Token {
    fn new(
        token_type: TokenType,
        lexeme: impl Into<String>,
        literal: Literal,
        line: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme: lexeme.into(),
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

    fn scan_tokens(mut self) -> Result<Tokens, Error> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens
            .push(Token::new(TokenType::Eof, "", Literal::None, self.line));
        Ok(Tokens {
            tokens: self.tokens,
        })
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, Literal::None);
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Literal) {
        let text = self.source[self.start..self.current]
            .iter()
            .collect::<String>();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line))
    }

    fn scan_token(&mut self) {
        match self.advance() {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::SemiColon),
            '*' => self.add_token(TokenType::Star),
            _ => unimplemented!(),
        }
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
    #[test]
    fn single_character() {
        let scanner = Scanner::new("(){},.-+;*");
        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens.unwrap().tokens,
            vec![
                Token::new(TokenType::LeftParen, "(", Literal::None, 1),
                Token::new(TokenType::RightParen, ")", Literal::None, 1),
                Token::new(TokenType::LeftBrace, "{", Literal::None, 1),
                Token::new(TokenType::RightBrace, "}", Literal::None, 1),
                Token::new(TokenType::Comma, ",", Literal::None, 1),
                Token::new(TokenType::Dot, ".", Literal::None, 1),
                Token::new(TokenType::Minus, "-", Literal::None, 1),
                Token::new(TokenType::Plus, "+", Literal::None, 1),
                Token::new(TokenType::SemiColon, ";", Literal::None, 1),
                Token::new(TokenType::Star, "*", Literal::None, 1),
                Token::new(TokenType::Eof, "", Literal::None, 1)
            ]
        )
    }
}
