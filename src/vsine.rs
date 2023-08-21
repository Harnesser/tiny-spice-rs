// Sinusoidal Current Source

use crate::circuit::{NodeId};
use std::f64::consts::PI;

#[derive(Clone)]
pub struct VoltageSourceSine {
    pub p: NodeId,
    pub n: NodeId,
    pub vo: f64, // offset (A)
    pub va: f64, // amplitude (A)
    pub freq: f64, // frequency (HZ)
    pub idx: usize,
}

impl VoltageSourceSine {

    // calculate the value at a certain time
    pub fn evaluate(&self, t: f64) -> f64 {
        self.vo + self.va * (2.0 * PI * self.freq * t).sin()
    }

}
    
