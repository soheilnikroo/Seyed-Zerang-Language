use std::fs::OpenOptions;

use crate::ast::Operator::{self, *};
use crate::ast::{
    AST,
    Expr::{self, *},
};

#[derive(Debug, PartialEq)]
pub enum ZerangValue {
    ZNil,
    ZBoolean(bool),
    ZNumber(f64),
    ZString(String),
}

impl ZerangValue {
    pub fn is_truthy(&self) -> bool {
        match self {
            ZNil | ZBoolean(false) => false,
            _ => true,
        }
    }
}

use ZerangValue::*;

pub type Output = ZerangValue;

#[derive(Debug)]
pub enum Error {
    ZeroDivision,
    UnsupportedOp(ZerangValue, Operator, ZerangValue),
    UnsupportedUnaryOp(Operator, ZerangValue),
}

pub fn evaluate(ast: AST) -> Result<Output, Error> {
    println!("Evaluating");
    evaluate_expression(&ast.top)
}

pub fn evaluate_expression(expr: &Expr) -> Result<ZerangValue, Error> {
    Ok(match expr {
        ENumber { value } => ZNumber(value.parse().unwrap()),
        EString { value } => ZString(value.clone()),
        EBool { value } => ZBoolean(*value),
        ENil => ZNil,
        EBinary {
            left,
            operator,
            right,
        } => {
            let lv = evaluate_expression(left)?;
            let rv = evaluate_expression(right)?;
            match (lv, operator, rv) {
                // Numeric operations
                (ZNumber(x), OAdd, ZNumber(y)) => ZNumber(x + y),
                (ZNumber(x), OSub, ZNumber(y)) => ZNumber(x - y),
                (ZNumber(x), OMul, ZNumber(y)) => ZNumber(x * y),
                (ZNumber(x), ODiv, ZNumber(y)) => {
                    if y == 0.0 {
                        return Err(Error::ZeroDivision);
                    } else {
                        ZNumber(x / y)
                    }
                }
                (ZNumber(x), OLt, ZNumber(y)) => ZBoolean(x < y),
                (ZNumber(x), OLe, ZNumber(y)) => ZBoolean(x <= y),
                (ZNumber(x), OGt, ZNumber(y)) => ZBoolean(x > y),
                (ZNumber(x), OGe, ZNumber(y)) => ZBoolean(x >= y),

                // String operations
                (ZString(x), OAdd, ZString(y)) => ZString(format!("{x}{y}")),

                // Equality works with any combination of values
                (x, OEq, y) => ZBoolean(x == y),
                (x, ONe, y) => ZBoolean(x != y),

                (lv, operator, rv) => {
                    return Err(Error::UnsupportedOp(lv, *operator, rv));
                }
            }
        }
        EUnary { operator, right } => {
            let rv = evaluate_expression(right)?;
            match (operator, rv) {
                (OSub, ZNumber(x)) => ZNumber(-x),
                (ONot, x) => ZBoolean(!x.is_truthy()),
                (operator, rv) => {
                    return Err(Error::UnsupportedUnaryOp(*operator, rv));
                }
            }
        }
        EGrouping { expression } => evaluate_expression(expression)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        assert_eq!(true, true)
    }
}
