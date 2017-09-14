// Sinusoidal Current Source

use circuit::{NodeId};

#[derive(Clone)]
pub struct CurrentSourceSine {
    pub p: NodeId,
    pub n: NodeId,
    pub vo: f32, // offset (A)
    pub va: f32, // amplitude (A)
    pub freq: f32, // frequency (HZ)
}

impl CurrentSourceSine {

    // calculate the value at a certain time
    pub fn evaluate(&self, t: f32) -> f32 {
        self.vo + self.va * (2.0 * 3.142 * self.freq * t).sin()
    }

}
    
