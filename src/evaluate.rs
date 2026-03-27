use crate::ast::Operator::*;
use crate::ast::{
    AST,
    Expr::{self, *},
};

#[derive(Debug)]
pub enum ZerangValue {
    ZNil,
    ZBoolean(bool),
    ZNumber(f64),
    ZString(String),
}

use ZerangValue::*;

pub type Output = ZerangValue;

#[derive(Debug)]
pub struct Error {}

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

                // String operations
                (ZString(x), OAdd, ZString(y)) => ZString(format!("{x}{y}")),

                _ => panic!("Unsupported operation"),
            }
        }
        EUnary { operator, right } => {
            todo!()
        }
        EGrouping { expression } => {
            todo!()
        }
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
