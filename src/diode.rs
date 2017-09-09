use circuit::{NodeId, BOLTZMANN, CHARGE, GMIN};

pub struct Diode {
    pub p: NodeId,
    pub n: NodeId,
    pub i_sat: f32,
    pub tdegc: f32,
}

impl Diode {

    // http://dev.hypertriton.com/edacious/trunk/doc/lec.pdf
    // page 4 of 14
    pub fn linearize(&self, v_d: f32) -> (f32, f32) {
        let v_thermal = BOLTZMANN * (363.0 + self.tdegc) / CHARGE;
        let mut i_d = self.i_sat * ( (v_d / v_thermal).exp() - 1.0 );
        if !i_d.is_finite() {
            println!("*WARNING* Possibly bad I_d {}", i_d);
            i_d = self.i_sat;
        }
        let g_eq = i_d / v_thermal;
        let i_eq = i_d - (g_eq * v_d);
        println!("*INFO* diode stamp: v_d = {}, g_eq = {}, i_eq = {}",
                 v_d, g_eq, i_eq);
        if !g_eq.is_finite() || !i_eq.is_finite() {
            println!("*ERROR* - banjaxed");
        }
        (g_eq, i_eq)
    }
}
