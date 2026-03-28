use std::fmt::Display;

use crate::ast::Operator::{self, *};
use crate::ast::Statement;
use crate::ast::{
    AST,
    Expr::{self, *},
};
use ZerangValue::*;

#[derive(Debug, PartialEq, Clone)]
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

impl Display for ZerangValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ZNil => f.write_str("nil"),
            ZBoolean(value) => f.write_str(&format!("{value}")),
            ZNumber(value) => f.write_str(&format!("{value}")),
            ZString(value) => f.write_str(value),
        }?;
        Ok(())
    }
}

pub type Output = ();

type Environment = crate::environment::Environment<ZerangValue>;

#[derive(Debug)]
pub enum Error {
    ZeroDivision,
    UnsupportedBinOp(ZerangValue, Operator, ZerangValue),
    UnsupportedUnaryOp(Operator, ZerangValue),
    NotFound(String),
}

pub struct Interpreter {
    top_level: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            top_level: Environment::new(),
        }
    }

    pub fn evaluate(&mut self, ast: AST) -> Result<Output, Error> {
        execute_statements(&ast.top, &mut self.top_level)?;
        Ok(())
    }
}

pub fn evaluate(ast: AST) -> Result<Output, Error> {
    let mut environment = Environment::new();
    execute_statements(&ast.top, &mut environment)?;
    Ok(())
}

pub fn execute_statements(
    statements: &Vec<Statement>,
    environment: &mut Environment,
) -> Result<(), Error> {
    for stmt in statements.iter() {
        execute_statement(stmt, environment)?
    }

    Ok(())
}

pub fn execute_statement(
    statement: &Statement,
    environment: &mut Environment,
) -> Result<(), Error> {
    match statement {
        Statement::SPrint { expr } => {
            let value = evaluate_expression(expr, environment)?;
            println!("{value}");
        }
        Statement::SExpression { expr } => {
            evaluate_expression(expr, environment)?;
        }
        Statement::SVarDecl { name, initializer } => {
            let iv = match initializer {
                Some(v) => evaluate_expression(v, environment)?,
                None => ZNil,
            };
            environment.declare(name, iv);
        }
    }

    Ok(())
}

pub fn evaluate_expression(expr: &Expr, environment: &Environment) -> Result<ZerangValue, Error> {
    Ok(match expr {
        ENumber { value } => ZNumber(value.parse().unwrap()),
        EString { value } => ZString(value.clone()),
        EBool { value } => ZBoolean(*value),
        ENil => ZNil,
        EVariable { name } => environment.lookup(name).unwrap().clone(),
        EBinary {
            left,
            operator,
            right,
        } => {
            let lv = evaluate_expression(left, environment)?;
            let rv = evaluate_expression(right, environment)?;
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
                    return Err(Error::UnsupportedBinOp(lv, *operator, rv));
                }
            }
        }
        EUnary { operator, right } => {
            let rv = evaluate_expression(right, environment)?;
            match (operator, rv) {
                (OSub, ZNumber(x)) => ZNumber(-x),
                (ONot, x) => ZBoolean(!x.is_truthy()),
                (operator, rv) => {
                    return Err(Error::UnsupportedUnaryOp(*operator, rv));
                }
            }
        }
        EGrouping { expression } => evaluate_expression(expression, environment)?,
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
