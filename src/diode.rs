use circuit::NodeId;
use newton_raphson::{Differentiable, DifferentiableTerm, DifferentiableEqn};

struct Diode {
    tdegc: f32,
}

impl Diode {
    const BOLTZMANN: f32 = 1.3806488e-23;
    const CHARGE: f32 = 1.603e-19;
}
