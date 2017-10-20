use circuit::{NodeId, BOLTZMANN, CHARGE, GMIN};

#[derive(Clone)]
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

        // thermal voltage. Should be ~26mV at room temperature
        let v_thermal = BOLTZMANN * (363.0 + self.tdegc) / CHARGE;

        // voltage alert
        if v_d < -1000.0 || v_d > 0.81 {
           println!("*WARNING* -1000 < v_d < 0.81 does not hold: {}", v_d);
        }

        let mut v_d_i = v_d;
        if v_d > 0.8 {
            v_d_i = 0.8;
        }

        // current through the diode, given the bias voltage
        let exp_vd_over_vt =(v_d_i / v_thermal).exp();

        let i_d = self.i_sat * ( exp_vd_over_vt - 1.0 );

        // calculate the diode companion model parameters
        // companion model is a current source in parallel with a resistor
        let mut g_eq: f32;
        let i_eq: f32;
        if i_d.is_finite() {

            // Equivalent conductance, limited to help convergence
            g_eq = (self.i_sat / v_thermal) * exp_vd_over_vt;
            if g_eq < GMIN {
                g_eq = GMIN;
            }

            // equivalent current source to pick up the slack
            i_eq = i_d - (g_eq * v_d_i);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curve_trace() {
        const VMAX: f32 = 5.0;
        const POINTS: i32 = 100;
        let diode = Diode{p:0, n:1, i_sat:1e-12, tdegc:27.0};
        println!("DATA pt Vd G_eq I_eq I_d");
        println!("DATA int V S A A");
        for pt in -POINTS..POINTS {
            let v_d = pt as f32 * (VMAX/POINTS as f32);
            let (g_eq, i_eq) = diode.linearize(v_d);
            let i_d = (v_d * g_eq) + i_eq;
            println!("DATA {} {} {} {} {}", pt, v_d, g_eq, i_eq, i_d);
        }
    }

}
