use std::cell::Cell;
use crate::circuit::{NodeId, BOLTZMANN, CHARGE, GMIN};

#[derive(Clone)]
pub struct Diode {
    pub p: NodeId,
    pub n: NodeId,
    pub i_sat: f64,
    pub tdegc: f64,
    v_thermal: f64,
    v_crit: f64,
    v_d_prev: Cell<f64>,
    i_d_prev: Cell<f64>,
    g_eq_prev: Cell<f64>,
}

impl Diode {

    pub fn new(p :NodeId, n :NodeId, i_sat :f64, tdegc: f64) -> Diode {
        let mut d = Diode {
            p,
            n,
            i_sat,
            tdegc,
            v_thermal: 0.0,
            v_crit: 0.0,
            v_d_prev: Cell::new(0.0),
            i_d_prev: Cell::new(0.0),
            g_eq_prev: Cell::new(GMIN),
        };
        d.update_v_thermal();
        d.update_v_crit();
        d
    }


    // http://dev.hypertriton.com/edacious/trunk/doc/lec.pdf
    // page 4 of 14
    pub fn linearize(&self, v_hat: f64, _: f64) -> (f64, f64) {
       
        // limit the excursion, following Colon via Nagel
        let v_d_prev = self.v_d_prev.get();
        let v_delta = v_hat - v_d_prev;

        let v_d_i :f64;
        if v_hat < self.v_crit {
            v_d_i = v_hat;
        } else if v_delta.abs() <= 2.0 * self.v_thermal {
            v_d_i = v_hat;
        } else if v_d_prev <= 0.0 {
            v_d_i = self.v_thermal * (v_hat / self.v_thermal).ln();
        } else {

            let arg :f64 = 1.0 + (v_delta / self.v_thermal);
            if arg <= 0.0 {
                v_d_i  = self.v_crit;
            } else {
                v_d_i = v_d_prev + self.v_thermal * arg.ln();
            }
            println!("*DIODE* v_d {}, v_d_prev {}", v_hat, v_d_prev);
            println!("*DIODE* arg {}, v_d_i {}", arg, v_d_i);
        }
        println!("*DIODE* V_d from {} V to {} V (v_crit={})", v_hat, v_d_i, self.v_crit);

        // current through the diode, given the bias voltage
        let exp_vd_over_vt =(v_d_i / self.v_thermal).exp();

        let i_d = self.i_sat * ( exp_vd_over_vt - 1.0 );

        // calculate the diode companion model parameters
        // companion model is a current source in parallel with a resistor
        let mut g_eq: f64;
        let i_eq: f64;
        if i_d.is_finite() {

            // Equivalent conductance, limited to help convergence
            g_eq = (self.i_sat / self.v_thermal) * exp_vd_over_vt;
            if g_eq < GMIN {
                g_eq = GMIN;
            }

            // equivalent current source to pick up the slack
            i_eq = i_d - (g_eq * v_d_i);

        } else {
            panic!("*FATAL* Possibly bad I_d {}", i_d);
        }

        // check that the companion model variables reasonable before
        // returning them to the main loop.
        if !g_eq.is_finite() || !i_eq.is_finite() {
            println!("*ERROR* - banjaxed");
        }

        self.v_d_prev.set(v_d_i);
        self.i_d_prev.set(i_d);
        self.g_eq_prev.set(g_eq);
        (g_eq, i_eq)
    }

    /// thermal voltage. Should be ~26mV at room temperature
    fn update_v_thermal(&mut self) {
        self.v_thermal = BOLTZMANN * (363.0 + self.tdegc) / CHARGE;
    }

    /// critical voltage. Colon limiting method
    /// See Nagel, section 5
    fn update_v_crit(&mut self) {
        // critical voltage for Colon
        self.v_crit = 
            self.v_thermal 
            * ( self.v_thermal / ( (2.0 as f64).sqrt() * self.i_sat ) )
            .ln();
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curve_trace() {
        const VMAX: f64 = 5.0;
        const POINTS: i32 = 100;
        let diode = Diode::new(0, 1, 1e-12, 27.0);
        println!("DATA pt Vd G_eq I_eq I_d");
        println!("DATA int V S A A");
        for pt in -POINTS..POINTS {
            let v_d = pt as f64 * (VMAX/POINTS as f64);
            let (g_eq, i_eq) = diode.linearize(v_d, v_d);
            let i_d = (v_d * g_eq) + i_eq;
            println!("DATA {} {} {} {} {}", pt, v_d, g_eq, i_eq, i_d);
        }
    }

}
