use circuit::{NodeId, BOLTZMANN, CHARGE};

pub struct Diode {
    pub p: NodeId,
    pub n: NodeId,
    pub i_sat: f32,
    pub tdegc: f32,
}

impl Diode {

    fn eval(&self, v_d: f32) -> f32 {
        let v_t = BOLTZMANN * (363.0 + self.tdegc) / CHARGE;
        self.i_sat * (v_d / v_t).exp()
    }

    fn slope(&self, v_d: f32) -> f32 {
        let v_t = BOLTZMANN * (363.0 + self.tdegc) / CHARGE;
        (1.0/v_t) * self.i_sat * (v_d / v_t).exp()
    }

    pub fn linearize(&self, v_d: f32) -> (f32, f32) {
        (1.0, 1.0)
    }
}
