extern crate tiny_spice;

use tiny_spice::diode::Diode;
use tiny_spice::newton_raphson::{Differentiable, DifferentiableEqn};

struct Constant {
    pub val: f32,
}
impl Differentiable for Constant {
    fn eval(&self, x: f32) -> f32 {
        self.val
    }

    fn slope(&self, x: f32) -> f32 {
        0.0
    }
}


struct Linear {
    pub gradient: f32,
}
impl Differentiable for Linear {
    fn eval(&self, x: f32) -> f32 {
        self.gradient * x
    }

    fn slope(&self, x: f32) -> f32 {
        self.gradient
    }
}

fn diode_resistor_isrc() -> DifferentiableEqn {

    let d1 = Diode {
        tdegc: 27.0,
        i_sat: 1.0e-9,
    };

    let i1 = Constant {
        val: 5.0,
    };

    let r1 = Linear {
        gradient: 0.5,
    };

    let mut cde = DifferentiableEqn {
        eqns: vec![],
    };

    cde.eqns.push(Box::new(i1));
    cde.eqns.push(Box::new(d1));
    cde.eqns.push(Box::new(r1));

    cde
}

#[test]
fn basic_eval() {
    let cde = diode_resistor_isrc();
    let answer = cde.eval(0.6);

    assert!(answer == 0.0, "Answer was {}", answer);
}

#[test]
fn basic_slope() {
    let cde = diode_resistor_isrc();
    let answer = cde.slope(0.6);

    assert!(answer == 0.0, "Answer was {}", answer);
}
