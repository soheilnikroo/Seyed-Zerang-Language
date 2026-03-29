use crate::reader::Source;
use TokenType::*;
use std::iter::Peekable;
use std::ops::Range;
use std::str::CharIndices;

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

    TIgnore,
    TEof,
}

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub line: usize,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, lexeme: &'a str, line: usize) -> Token<'a> {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}

#[derive(Debug)]
pub struct Tokens<'a> {
    pub tokens: Vec<Token<'a>>,
}

#[derive(Debug)]
pub enum ScanError {
    UnexpectedCharacter { line: usize, ch: char },
    UnterminatedString { line: usize },
}

#[derive(Debug)]
pub struct Error(Vec<ScanError>);

impl Error {
    pub fn iter(&self) -> std::slice::Iter<'_, ScanError> {
        self.0.iter()
    }
}

type Chars<'a> = Peekable<CharIndices<'a>>;

fn accept(
    chars: &mut Chars,
    token_type: TokenType,
    start: usize,
) -> Option<(TokenType, Range<usize>)> {
    let (n, _) = chars.next()?;
    Some((token_type, start..n + 1))
}

fn peek(chars: &mut Chars, ch: char) -> bool {
    if let Some(&(_, c)) = chars.peek() {
        c == ch
    } else {
        false
    }
}

fn map_keyword(lexeme: &str) -> TokenType {
    match lexeme {
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
    }
}

fn scan_tokens<'a>(source: &'a String) -> Result<Tokens<'a>, Error> {
    let mut chars = source.char_indices().peekable();
    let mut tokens = Vec::new();
    let line = 1;
    while let Some((mut toktype, range)) = scan_token(&mut chars) {
        if toktype == TIgnore {
            continue;
        }
        let lexeme = &source[range];
        if toktype == TIdentifier {
            toktype = map_keyword(lexeme);
        }
        tokens.push(Token::new(toktype, lexeme, line));
    }
    tokens.push(Token::new(TEof, "", line));
    Ok(Tokens { tokens })
}

fn scan_token(chars: &mut Chars) -> Option<(TokenType, Range<usize>)> {
    scan_simple_symbol(chars)
        .or_else(|| scan_compare_symbol(chars))
        .or_else(|| ignore_whitespace(chars))
        .or_else(|| scan_number(chars))
        .or_else(|| scan_identifier(chars))
        .or_else(|| scan_string(chars))
}

fn ignore_whitespace(chars: &mut Chars) -> Option<(TokenType, Range<usize>)> {
    let &(start, ch) = chars.peek()?;
    let mut end = start + 1;
    if ch.is_whitespace() {
        while let Some(&(n, ch)) = chars.peek() {
            if ch.is_whitespace() {
                end = n;
                let _ = chars.next();
            } else {
                break;
            }
        }
        Some((TIgnore, start..end + 1))
    } else {
        None
    }
}

fn scan_simple_symbol(chars: &mut Chars) -> Option<(TokenType, Range<usize>)> {
    let &(start, ch) = chars.peek()?;
    match ch {
        '+' => accept(chars, TPlus, start),
        '-' => accept(chars, TMinus, start),
        '*' => accept(chars, TStar, start),
        '(' => accept(chars, TLeftParen, start),
        ')' => accept(chars, TRightParen, start),
        '{' => accept(chars, TLeftBrace, start),
        '}' => accept(chars, TRightBrace, start),
        ';' => accept(chars, TSemiColon, start),
        ',' => accept(chars, TComma, start),
        '.' => accept(chars, TDot, start),
        '/' => {
            chars.next().unwrap();
            if peek(chars, '/') {
                let mut end = start;
                while let Some((n, ch)) = chars.next() {
                    end = n;
                    if ch == '\n' {
                        break;
                    }
                }
                Some((TIgnore, start..end))
            } else {
                Some((TSlash, start..start + 1))
            }
        }
        _ => None,
    }
}

fn scan_compare_symbol(chars: &mut Chars) -> Option<(TokenType, Range<usize>)> {
    let &(start, ch) = chars.peek()?;
    match ch {
        '<' => {
            let _ = chars.next();
            if peek(chars, '=') {
                accept(chars, TLessEqual, start)
            } else {
                Some((TLess, start..start + 1))
            }
        }
        '>' => {
            let _ = chars.next();
            if peek(chars, '=') {
                accept(chars, TGreaterEqual, start)
            } else {
                Some((TGreater, start..start + 1))
            }
        }
        '=' => {
            let _ = chars.next();
            if peek(chars, '=') {
                accept(chars, TEqualEqual, start)
            } else {
                Some((TEqual, start..start + 1))
            }
        }
        '!' => {
            let _ = chars.next();
            if peek(chars, '=') {
                accept(chars, TBangEqual, start)
            } else {
                Some((TBang, start..start + 1))
            }
        }
        _ => None,
    }
}

fn scan_number(chars: &mut Chars) -> Option<(TokenType, Range<usize>)> {
    let &(start, ch) = chars.peek()?;
    let mut end = start;
    if ch.is_digit(10) {
        while let Some(&(n, ch)) = chars.peek() {
            if ch.is_digit(10) {
                end = n;
                chars.next().unwrap();
            } else {
                break;
            }
        }
        if peek(chars, '.') {
            chars.next().unwrap();
            while let Some(&(n, ch)) = chars.peek() {
                if ch.is_digit(10) {
                    end = n;
                    chars.next().unwrap();
                } else {
                    break;
                }
            }
        }
        Some((TNumber, start..end + 1))
    } else {
        None
    }
}

fn scan_string(chars: &mut Chars) -> Option<(TokenType, Range<usize>)> {
    let &(start, ch) = chars.peek()?;
    let mut end = start + 1;
    if ch == '"' {
        chars.next().unwrap();
        while let Some((n, ch)) = chars.next() {
            end = n;
            if ch == '"' {
                break;
            }
        }
        Some((TString, start..end + 1))
    } else {
        None
    }
}

fn scan_identifier(chars: &mut Chars) -> Option<(TokenType, Range<usize>)> {
    let &(start, ch) = chars.peek()?;
    let mut end = start;
    if ch.is_alphabetic() || ch == '_' {
        while let Some(&(n, ch)) = chars.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                end = n;
                chars.next().unwrap();
            } else {
                break;
            }
        }
        Some((TIdentifier, start..end + 1))
    } else {
        None
    }
}

pub fn tokenize<'a>(source: &'a Source) -> Result<Tokens<'a>, Error> {
    scan_tokens(&source.contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        assert_eq!(true, true);
    }
    #[test]
    fn single_character() {
        let source = String::from("(){},.-+;*/");
        let tokens = scan_tokens(&source);
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
                Token::new(TSlash, "/", 1),
                Token::new(TEof, "", 1),
            ]
        )
    }

    #[test]
    fn two_character() {
        let source = String::from("! != < <= > >= == =");
        let tokens = scan_tokens(&source);
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
                Token::new(TEof, "", 1),
            ]
        )
    }

    #[test]
    fn strings() {
        let source = String::from("\"hello\" \"world\"");
        let tokens = scan_tokens(&source);
        assert_eq!(
            tokens.unwrap().tokens,
            vec![
                Token::new(TString, "\"hello\"", 1),
                Token::new(TString, "\"world\"", 1),
                Token::new(TEof, "", 1),
            ]
        )
    }

    #[test]
    fn numbers() {
        let source = String::from("12345 123.45");
        let tokens = scan_tokens(&source);
        assert_eq!(
            tokens.unwrap().tokens,
            vec![
                Token::new(TNumber, "12345", 1),
                Token::new(TNumber, "123.45", 1),
                Token::new(TEof, "", 1),
            ]
        )
    }
    #[test]
    fn identifiers() {
        let source = String::from("abc def123 ab_cd");
        let tokens = scan_tokens(&source);
        assert_eq!(
            tokens.unwrap().tokens,
            vec![
                Token::new(TIdentifier, "abc", 1),
                Token::new(TIdentifier, "def123", 1),
                Token::new(TIdentifier, "ab_cd", 1),
                Token::new(TEof, "", 1),
            ]
        )
    }
    #[test]
    fn keywords() {
        let source = String::from(
            "and class else false for fun if nil or print return super this true var while",
        );
        let tokens = scan_tokens(&source);
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
                Token::new(TEof, "", 1),
            ]
        )
    }
}
