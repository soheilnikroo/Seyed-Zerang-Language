use crate::tokenize::{Literal, Token};

pub enum Expr {
    Num {
        value: f64,
    },
    Str {
        value: String,
    },
    Bool {
        value: bool,
    },
    Nil,
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
}

impl Expr {
    fn num(value: f64) -> Expr {
        Expr::Num { value }
    }

    fn str(value: impl Into<String>) -> Expr {
        Expr::Str {
            value: value.into(),
        }
    }

    fn bool(value: bool) -> Expr {
        Expr::Bool { value }
    }

    fn nil() -> Expr {
        Expr::Nil
    }
    fn binary(left: Expr, operator: Token, right: Expr) -> Expr {
        Expr::Binary {
            left: left.into(),
            operator,
            right: right.into(),
        }
    }

    fn unary(operator: Token, right: Expr) -> Expr {
        Expr::Unary {
            operator,
            right: right.into(),
        }
    }

    fn grouping(expression: Expr) -> Expr {
        Expr::Grouping {
            expression: expression.into(),
        }
    }
}

pub fn format_exp(e: &Expr) -> String {
    match e {
        Expr::Num { value } => format!("{value}"),
        Expr::Str { value } => format!("{value:?}"),
        Expr::Bool { value } => format!("{value}"),
        Expr::Nil => "nil".to_string(),
        Expr::Binary {
            left,
            operator,
            right,
        } => {
            format!(
                "({} {} {})",
                operator.lexeme,
                format_exp(left),
                format_exp(right)
            )
        }
        Expr::Unary { operator, right } => {
            format!("({} {})", operator.lexeme, format_exp(right))
        }
        Expr::Grouping { expression } => {
            format!("(group {})", format_exp(expression))
        }
    }
}

pub fn main() {}
