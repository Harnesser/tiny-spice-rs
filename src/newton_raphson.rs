use circuit::{NodeId};

pub trait Differentiable {
    fn eval(&self, x: f32) -> f32;
    fn slope(&self, x: f32) -> f32;
}

pub struct DifferentiableEqn {
    pub eqns: Vec<Box<Differentiable>>,
}

impl DifferentiableEqn {

    pub fn solve(&self, i: f32) -> Option<f32> {

        const EPSILON : f32 = 10e-14;
        const TOLERANCE : f32 = 10e-7;
        const MAX_ITERS : usize = 100;

        let mut x0 = i;
        let mut converged = false;

        println!("*INFO* Newton-Raphson solve");
        for i in 0..MAX_ITERS {
            let y = self.eval(x0);
            let yprime = self.slope(x0);

            println!("  [{:2}] x0 = {} y = {}  y' = {}", i, x0, y, yprime);

            // don't divice by too small of a number
            if !yprime.is_finite() || yprime.abs() < EPSILON {
                println!("*ERROR* Epsilon");
                break;
            }

            // Do Newton-Raphson
            // !!!FIXME!!! Horrible hack to aid diode convergence
            let mut twiddle = y / yprime;
            if twiddle > 0.2 {
                twiddle = 0.2;
            } else if twiddle < -0.2 {
                twiddle = -0.2;
            }
            let x1 = x0 - twiddle;
            println!("        twiddle = {} x1 = {}", twiddle, x1);

            // Have we found a result within the correct tolerance?
            if (x1-x0).abs() <= ( TOLERANCE * x1.abs() ) {
                converged = true;
                x0 = x1;
                println!("*INFO* Converged");
                break;
            }

            x0 = x1;
        }

        if converged {
            Some(x0)
        } else {
            println!("*ERROR* Divergent");
            None
        }
    }

    pub fn eval(&self, x: f32) -> f32 {
        let mut res = 0.0;
        for eqn in &self.eqns {
            res -= eqn.eval(x);
        }
        res
    }

    pub fn slope(&self, x: f32) -> f32 {
        let mut res = 0.0;
        for eqn in &self.eqns {
            res -= eqn.slope(x);
        }
        res
    }
}

