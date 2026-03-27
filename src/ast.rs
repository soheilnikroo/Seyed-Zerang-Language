#[derive(Debug)]
pub struct AST {
    pub top: Option<Expr>,
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    OAdd,
    OSub,
    OMul,
    ODiv,
    OLt,
    OLe,
    OGt,
    OGe,
    OEq,
    ONe,
    ONot,
    OAnd,
    OOr,
}

use Operator::*;

#[derive(Debug)]
pub enum Expr {
    ENumber {
        value: String,
    },
    EString {
        value: String,
    },
    EBool {
        value: bool,
    },
    ENil,
    EBinary {
        left: Box<Expr>,
        operator: Operator,
        right: Box<Expr>,
    },
    EUnary {
        operator: Operator,
        right: Box<Expr>,
    },
    EGrouping {
        expression: Box<Expr>,
    },
}

use Expr::*;

impl Expr {
    fn num(value: impl Into<String>) -> Expr {
        ENumber {
            value: value.into(),
        }
    }

    fn string(value: impl Into<String>) -> Expr {
        EString {
            value: value.into(),
        }
    }

    fn bool(value: bool) -> Expr {
        EBool { value }
    }

    fn nil() -> Expr {
        ENil
    }
    fn binary(left: Expr, operator: Operator, right: Expr) -> Expr {
        EBinary {
            left: left.into(),
            operator,
            right: right.into(),
        }
    }

    fn unary(operator: Operator, right: Expr) -> Expr {
        EUnary {
            operator,
            right: right.into(),
        }
    }

    fn grouping(expression: Expr) -> Expr {
        EGrouping {
            expression: expression.into(),
        }
    }
}

pub fn format_operator(operator: &Operator) -> &'static str {
    match operator {
        OAdd => "+",
        OSub => "-",
        OMul => "*",
        ODiv => "/",
        OLt => ">",
        OLe => "<=",
        OGt => ">",
        OGe => ">=",
        OEq => "==",
        ONe => "!=",
        ONot => "!",
        OAnd => "and",
        OOr => "or",
    }
}

pub fn format_exp(e: &Expr) -> String {
    match e {
        ENumber { value } => format!("{value}"),
        EString { value } => format!("{value:?}"),
        EBool { value } => format!("{value}"),
        ENil => "nil".to_string(),
        EBinary {
            left,
            operator,
            right,
        } => {
            format!(
                "({} {} {})",
                format_operator(operator),
                format_exp(left),
                format_exp(right)
            )
        }
        EUnary { operator, right } => {
            format!("({} {})", format_operator(operator), format_exp(right))
        }
        EGrouping { expression } => {
            format!("(group {})", format_exp(expression))
        }
    }
}
