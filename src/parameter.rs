//! Parameters

use crate::bracket_expression::{Expression};

/// Parameter
#[derive(Clone, Debug)]
pub struct Parameter {
    pub name: String,
    pub defval: Option<Expression>,
    pub expr: Option<Expression>,
    pub value: Option<f64>,
}

impl Parameter {

    /// goes in `expr`
    pub fn override_from_expression(name: &str, expr: &Expression) -> Self {
        Parameter {
            name: name.to_string(),
            defval: None,
            expr: Some(expr.clone()),
            value: None
        }
    }

    /// goes in `defval`
    pub fn default_from_expression(name: &str, expr: &Expression) -> Self {
        Parameter {
            name: name.to_string(),
            defval: Some(expr.clone()),
            expr: None,
            value: None
        }
    }

}

