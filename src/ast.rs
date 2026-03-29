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
    EAssign {
        name: String,
        value: Box<Expr>,
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

    pub fn assign(name: impl Into<String>, value: Expr) -> Expr {
        EAssign {
            name: name.into(),
            value: value.into(),
        }
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
    SBlock {
        statements: Vec<Statement>,
    },
    SIf {
        condition: Expr,
        consequence: Box<Statement>,
        alternative: Option<Box<Statement>>,
    },
    SWhile {
        condition: Expr,
        body: Box<Statement>,
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

    pub fn block(statements: Vec<Statement>) -> Statement {
        Self::SBlock { statements }
    }

    pub fn if_statement(
        condition: Expr,
        consequence: Statement,
        alternative: Option<Statement>,
    ) -> Statement {
        Self::SIf {
            condition,
            consequence: consequence.into(),
            alternative: alternative.map(|s| Box::new(s)),
        }
    }

    pub fn while_statement(condition: Expr, body: Statement) -> Statement {
        Self::SWhile {
            condition,
            body: body.into(),
        }
    }
}
