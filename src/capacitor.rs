use crate::circuit::{NodeId};

#[derive(Clone)]
pub struct Capacitor {
    pub ident: String,
    pub a: NodeId,
    pub b: NodeId,
    pub value: f64, // Farads
}

impl Capacitor {

    pub fn new(ident: &str, a: NodeId, b: NodeId, value: f64) -> Capacitor {
        Capacitor {
            ident: ident.to_string(),
            a,
            b,
            value,
        }
    }

    pub fn linearize(&self, v_prev: f64, t_delta: f64) -> (f64, f64) {
        // v_prev: voltage across cap at last solved timepoint
        // t_delta: change in time between now and last solved timepoint
        // implements backward-euler integration
        let g_eq = self.value / t_delta;
        let i_eq = g_eq * v_prev;
        (g_eq, i_eq)
    }

}

