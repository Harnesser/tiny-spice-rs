use circuit::NodeId;

pub trait Differentiable {
    fn eval(&self, x: f32) -> f32;
    fn slope(&self, x: f32) -> f32;
}

pub struct DifferentiableEqn {
    pub eqns: Vec<Box<Differentiable>>,
}

impl DifferentiableEqn {

    pub fn eval(&self, x: f32) -> f32 {
        let mut res = 0.0;
        for eqn in &self.eqns {
            res += eqn.eval(x);
        }
        res
    }

    pub fn slope(&self, x: f32) -> f32 {
        let mut res = 0.0;
        for eqn in &self.eqns {
            res += eqn.slope(x);
        }
        res
    }
}

