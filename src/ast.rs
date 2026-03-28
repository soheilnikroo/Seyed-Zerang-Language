use Expr::*;
use Operator::*;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct AST {
    pub top: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match self {
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
        })
    }
}

#[derive(Debug, PartialEq)]
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
    EVariable {
        name: String,
    },
}

impl Expr {
    pub fn number(value: impl Into<String>) -> Expr {
        ENumber {
            value: value.into(),
        }
    }

    pub fn string(value: impl Into<String>) -> Expr {
        EString {
            value: value.into(),
        }
    }

    pub fn bool(value: bool) -> Expr {
        EBool { value }
    }

    pub fn nil() -> Expr {
        ENil
    }
    pub fn binary(left: Expr, operator: Operator, right: Expr) -> Expr {
        EBinary {
            left: left.into(),
            operator,
            right: right.into(),
        }
    }

    pub fn unary(operator: Operator, right: Expr) -> Expr {
        EUnary {
            operator,
            right: right.into(),
        }
    }

    pub fn grouping(expression: Expr) -> Expr {
        EGrouping {
            expression: expression.into(),
        }
    }

    pub fn variable(name: impl Into<String>) -> Expr {
        EVariable { name: name.into() }
    }
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    SPrint {
        expr: Expr,
    },
    SExpression {
        expr: Expr,
    },
    SVarDecl {
        name: String,
        initializer: Option<Expr>,
    },
}

impl Statement {
    pub fn print(e: Expr) -> Self {
        Self::SPrint { expr: e }
    }

    pub fn expression(e: Expr) -> Self {
        Self::SExpression { expr: e }
    }

    pub fn var_decl(name: impl Into<String>, initializer: Option<Expr>) -> Self {
        Self::SVarDecl {
            name: name.into(),
            initializer,
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
        EVariable { name } => format!("{name}"),
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
