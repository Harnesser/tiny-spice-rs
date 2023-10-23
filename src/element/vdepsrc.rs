//! Voltage-Controlled Dependent Source Implementations
//!
//! * `E` - Voltage-Controlled Voltage Source (VCVS)
//! * `G` - Voltage-Controlled Current Source (VCCS)

use std::cell::Cell;

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
    pub idx: usize, // index of voltage source in "known" column
}

impl Vcvs {

    /// I need to be stamping a voltage source, somehow...
    pub fn evaluate(&self, v: f64) -> f64 {
        v * self.k
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
    v_prev: Cell<f64>,
}

impl Vccs {


    pub fn new(ident: &str, p: NodeId, n: NodeId, cp: NodeId, cn: NodeId, k:f64) -> Self {
        Vccs {
            ident:ident.to_string(),
            p, n, cp, cn, k,
            v_prev: Cell::new(0.0),
        }
    }

    /// I need to be stamping a voltage source, somehow...
    pub fn evaluate(&self, v: f64) -> f64 {
        let limit = 0.01;
        let v_delta = (v - self.v_prev.get()).abs();

        let mut v_ctrl = v;
        if v_delta > limit {
            v_ctrl = self.v_prev.get() + limit;
        } else if v_delta < -limit {
            v_ctrl = self.v_prev.get() - limit;
        }
        self.v_prev.set(v_ctrl);

        if (v_ctrl > 10e3) || (v_ctrl < -10e3) {
            panic!("nah. v_ctrl = {} V", v_ctrl);
        }

        v_ctrl * self.k
    }

}


