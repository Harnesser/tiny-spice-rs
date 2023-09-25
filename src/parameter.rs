//! Parameters

use crate::bracket_expression::{Expression};

/// Parameter
#[derive(Clone, Debug)]
pub struct Parameter {
    pub name: String,
    pub defval: Option<f64>,
    pub expr: Option<Expression>,
    pub value: Option<f64>,
}

impl Parameter {

    pub fn from_expression(name: &str, expr: &Expression) -> Self {
        Parameter {
            name: name.to_string(),
            defval: None,
            expr: Some(expr.clone()),
            value: None
        }
    }

}

