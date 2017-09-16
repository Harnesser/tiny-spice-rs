use circuit::{NodeId, GMIN};

pub struct Capacitor {
    pub a: NodeId,
    pub b: NodeId,
    pub value: f32, // Farads
}

