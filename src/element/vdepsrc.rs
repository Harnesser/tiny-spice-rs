//! Voltage-Controlled Dependent Source Implementations
//!
//! * `E` - Voltage-Controlled Voltage Source (VCVS)
//! * `G` - Voltage-Controlled Current Source (VCCS)

use crate::circuit::{NodeId};

/// `E` - VCVS
#[derive(Clone)]
pub struct Vcvs {
    pub ident: String,
    pub p: NodeId,
    pub n: NodeId,
    pub cp: NodeId,
    pub cn: NodeId,
    pub k: f64,
}

impl Vcvs {

    /// I need to be stamping a voltage source, somehow...
    pub fn evaluate(&self, t: f64) -> f64 {
        1.0
    }

}

/// `G` - VCCS
#[derive(Clone)]
pub struct Vccs {
    pub ident: String,
    pub p: NodeId,
    pub n: NodeId,
    pub cp: NodeId,
    pub cn: NodeId,
    pub k: f64,
}

impl Vccs {

    /// I need to be stamping a voltage source, somehow...
    pub fn evaluate(&self, t: f64) -> f64 {
        1.0
    }

}


