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
    pub fn linearize(&self, v_d_i: f32) -> (f32, f32) {

        // thermal voltage. Should be ~26mV at room temperature
        let v_thermal = BOLTZMANN * (363.0 + self.tdegc) / CHARGE;

        // solve convergence problems by limiting the positive voltage
        // allowed across the diode
        let mut v_d = v_d_i;
        if v_d > 0.8 {
            v_d = 0.8;
        }

        // current through the diode, given the bias voltage
        let i_d = self.i_sat * ( (v_d / v_thermal).exp() - 1.0 );

        // calculate the diode companion model parameters
        // companion model is a current source in parallel with a resistor
        let g_eq: f32;
        let i_eq: f32;
        if i_d.is_finite() {
            println!("{}V", v_thermal);
            g_eq = i_d / v_thermal;
            i_eq = i_d - (g_eq * v_d);
        } else {
            println!("*WARNING* Possibly bad I_d {}", i_d);
            g_eq = GMIN;
            i_eq = 0.0;
        }

        // check that the companion model variables reasonable before
        // returning them to the main loop.
        if !g_eq.is_finite() || !i_eq.is_finite() {
            println!("*ERROR* - banjaxed");
        }
        (g_eq, i_eq)
    }
}
