use circuit::NodeId;

pub trait Differentiable {
    fn eval(&self, x: f32) -> f32;
    fn slope(&self, x: f32) -> f32;
}

pub struct DifferentiableTerm {
    f: fn(f32) -> f32,
    fprime: fn(f32) -> f32,
}

impl Differentiable for DifferentiableTerm {

    fn eval(&self, x: f32) -> f32 {
        0.1
    }

    fn slope(&self, x: f32) -> f32 {
        100.0
    }

}


pub struct DifferentiableEqn {
    node: Option<NodeId>,
    eqns: Vec<DifferentiableTerm>,
}

impl DifferentiableEqn {

    pub fn eval(&self, x: f32) -> f32 {
        let mut res = 0.0;
        for eqn in &self.eqns {
            res += eqn.eval(x);
        }
        res
    }
}

