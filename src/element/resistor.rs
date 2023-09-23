//! Resistor Implementation

use crate::circuit::{NodeId};

/// Resistor Implementation
#[allow(dead_code)]
#[derive(Clone)]
pub struct Resistor {
    pub ident: String,
    pub a: NodeId,
    pub b: NodeId,
    pub value: f64, // Ohms
}

