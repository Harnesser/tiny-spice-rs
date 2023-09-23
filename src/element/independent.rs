//! Independent Source Implementations

use crate::circuit::NodeId;

/// Current Source Implementation
#[allow(dead_code)]
#[derive(Clone)]
pub struct CurrentSource {
    pub p: NodeId,
    pub n: NodeId,
    pub value: f64, // Amperes
}

/// Voltage Source Implementation
#[allow(dead_code)]
#[derive(Clone)]
pub struct VoltageSource {
    pub p: NodeId,
    pub n: NodeId,
    pub value: f64, // Volts
    pub idx: usize, // index of voltage source in "known" column
}

