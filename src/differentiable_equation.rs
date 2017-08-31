
pub trait DifferentiableEquation {
    pub fn eval(&self, x: f32) -> f32;
    pub fn slope(&self, x: f32) -> f32;
}

struct DifferentiableEquation {

}

impl 
