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

    /*
    pub fn resolve(&self) -> f64 {
        if let Some(val) = self.value {
            val
        } else if let Some(self.expr) {
            // evaluate
            -100.00
        } else if let Some(val) = self.defval {
            val
        } else {
            println!("*ERROR* Can't resolve parameter to a value");
        }
    }
*/

}


