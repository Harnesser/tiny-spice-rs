use circuit::{NodeId, GMIN};

#[derive(Clone)]
pub struct Capacitor {
    pub a: NodeId,
    pub b: NodeId,
    pub value: f32, // Farads
}

impl Capacitor {

    pub fn linearize(&self, v: f32, t: f32) -> (f32, f32) {
        let g_eq = self.value / t;
        let i_eq = g_eq * v;
        (g_eq, i_eq)
    }

}

