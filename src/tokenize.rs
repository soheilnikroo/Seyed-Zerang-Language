use crate::reader::Source;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    TLeftParen,
    TRightParen,
    TLeftBrace,
    TRightBrace,
    TComma,
    TDot,
    TMinus,
    TPlus,
    TSemiColon,
    TSlash,
    TStar,

    // One or two character tokens
    TBang,
    TBangEqual,
    TEqual,
    TEqualEqual,
    TGreater,
    TGreaterEqual,
    TLess,
    TLessEqual,

    // Literals
    TIdentifier,
    TString,
    TNumber,

    // Keywords
    TAnd,
    TClass,
    TElse,
    TFalse,
    TFun,
    TFor,
    TIf,
    TNil,
    TOr,
    TPrint,
    TReturn,
    TSuper,
    TThis,
    TTrue,
    TVar,
    TWhile,

    TEof,
}

use TokenType::*;

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
    pub line: usize,
}

impl Token {
    fn new(token_type: TokenType, lexeme: impl Into<String>, line: usize) -> Self {
        Self {
            token_type,
            lexeme: lexeme.into(),
            line,
        }
    }
}

#[derive(Debug)]
pub struct Tokens {
    pub tokens: Vec<Token>,
}

#[derive(Debug)]
pub struct Error(Vec<ScanError>);

#[derive(Debug)]
enum ScanError {
    UnexpectedCharacter { line: usize, ch: char },
}

struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    errors: Vec<ScanError>,
}

impl Scanner {
    fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            errors: Vec::new(),
        }
    }

    fn error(&mut self, err: ScanError) {
        self.errors.push(err);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_tokens(mut self) -> Result<Tokens, Error> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(TEof, "", self.line));

        if self.errors.len() == 0 {
            Ok(Tokens {
                tokens: self.tokens,
            })
        } else {
            Err(Error(self.errors))
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn lexeme(&self) -> String {
        self.source[self.start..self.current].iter().collect()
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, Literal::None);
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Literal) {
        self.tokens
            .push(Token::new(token_type, self.lexeme(), self.line))
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\x00'
        } else {
            self.source[self.current]
        }
    }

    fn scan_token(&mut self) {
        match self.advance() {
            '(' => self.add_token(TLeftParen),
            ')' => self.add_token(TRightParen),
            '{' => self.add_token(TLeftBrace),
            '}' => self.add_token(TRightBrace),
            ',' => self.add_token(TComma),
            '.' => self.add_token(TDot),
            '-' => self.add_token(TMinus),
            '+' => self.add_token(TPlus),
            ';' => self.add_token(TSemiColon),
            '*' => self.add_token(TStar),
            '!' => {
                let token_type = if self.matches('=') { TBangEqual } else { TBang };
                self.add_token(token_type);
            }
            '=' => {
                let token_type = if self.matches('=') {
                    TEqualEqual
                } else {
                    TEqual
                };
                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.matches('=') { TLessEqual } else { TLess };
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.matches('=') {
                    TGreaterEqual
                } else {
                    TGreater
                };
                self.add_token(token_type);
            }
            '/' => {
                if self.matches('/') {
                    while self.peek() != '\n' && self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TSlash);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            char if char.is_digit(10) => {
                self.number();
            }
            char if char.is_alphabetic() => self.identifier(),
            char => {
                self.error(ScanError::UnexpectedCharacter {
                    line: self.line,
                    ch: char,
                });
            }
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            todo!("Unterminated String");
        }
        self.advance();
        let value = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect::<String>();
        self.add_token_with_literal(TString, Literal::Str(value));
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }
        let literal = Literal::Num(self.lexeme().parse().unwrap());
        self.add_token_with_literal(TNumber, literal);
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let token_type = match &self.lexeme()[..] {
            "and" => TAnd,
            "class" => TClass,
            "else" => TElse,
            "false" => TFalse,
            "for" => TFor,
            "fun" => TFun,
            "if" => TIf,
            "nil" => TNil,
            "or" => TOr,
            "print" => TPrint,
            "return" => TReturn,
            "super" => TSuper,
            "this" => TThis,
            "true" => TTrue,
            "var" => TVar,
            "while" => TWhile,
            _ => TIdentifier,
        };

        self.add_token(token_type);
    }
}

pub fn tokenize(source: Source) -> Result<Tokens, Error> {
    println!("Tokenizing");
    Scanner::new(&source.contents).scan_tokens()
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
                Token::new(TLeftParen, "(", 1),
                Token::new(TRightParen, ")", 1),
                Token::new(TLeftBrace, "{", 1),
                Token::new(TRightBrace, "}", 1),
                Token::new(TComma, ",", 1),
                Token::new(TDot, ".", 1),
                Token::new(TMinus, "-", 1),
                Token::new(TPlus, "+", 1),
                Token::new(TSemiColon, ";", 1),
                Token::new(TStar, "*", 1),
                Token::new(TEof, "", 1)
            ]
        )
    }

    #[test]
    fn two_character() {
        let scanner = Scanner::new("! != < <= > >= == =");
        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens.unwrap().tokens,
            vec![
                Token::new(TBang, "!", 1),
                Token::new(TBangEqual, "!=", 1),
                Token::new(TLess, "<", 1),
                Token::new(TLessEqual, "<=", 1),
                Token::new(TGreater, ">", 1),
                Token::new(TGreaterEqual, ">=", 1),
                Token::new(TEqualEqual, "==", 1),
                Token::new(TEqual, "=", 1),
                Token::new(TEof, "", 1)
            ]
        )
    }

    #[test]
    fn strings() {
        let scanner = Scanner::new("\"hello\" \"world\"");
        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens.unwrap().tokens,
            vec![
                Token::new(TString, "\"hello\"", 1),
                Token::new(TString, "\"world\"", 1),
                Token::new(TEof, "", 1)
            ]
        )
    }

    #[test]
    fn numbers() {
        let scanner = Scanner::new("12345 123.45");
        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens.unwrap().tokens,
            vec![
                Token::new(TNumber, "12345", 1),
                Token::new(TNumber, "123.45", 1),
                Token::new(TEof, "", 1)
            ]
        )
    }

    #[test]
    fn identifiers() {
        let scanner = Scanner::new("abc def123 ab_cd");
        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens.unwrap().tokens,
            vec![
                Token::new(TIdentifier, "abc", 1),
                Token::new(TIdentifier, "def123", 1),
                Token::new(TIdentifier, "ab_cd", 1),
                Token::new(TEof, "", 1)
            ]
        )
    }

    #[test]
    fn keywords() {
        let scanner = Scanner::new(
            "and class else false for fun if nil or print return super this true var while",
        );
        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens.unwrap().tokens,
            vec![
                Token::new(TAnd, "and", 1),
                Token::new(TClass, "class", 1),
                Token::new(TElse, "else", 1),
                Token::new(TFalse, "false", 1),
                Token::new(TFor, "for", 1),
                Token::new(TFun, "fun", 1),
                Token::new(TIf, "if", 1),
                Token::new(TNil, "nil", 1),
                Token::new(TOr, "or", 1),
                Token::new(TPrint, "print", 1),
                Token::new(TReturn, "return", 1),
                Token::new(TSuper, "super", 1),
                Token::new(TThis, "this", 1),
                Token::new(TTrue, "true", 1),
                Token::new(TVar, "var", 1),
                Token::new(TWhile, "while", 1),
                Token::new(TEof, "", 1)
            ]
        )
    }
}
