//! Piecewise Linear Voltage Source Implementation

use crate::circuit::{NodeId};

#[derive(Clone,Debug)]
pub struct VoltageSourcePwl {
    pub p: NodeId,
    pub n: NodeId,
    pub pat: Vec<(f64, f64)>, // (time, val)
    pub t_delay: f64, // delay
    pub repeat: f64, // repeat spec
    pub idx: usize,
}

impl VoltageSourcePwl {

    // calculate the value at a certain time
    //
    // figure out where we are in the cycle.
    // then interpolate the values?
    pub fn evaluate(&self, t: f64) -> f64 {
        if self.pat.is_empty() {
            return 0.0;
        }

        let idx_last = self.pat.len() - 1;
        let t_period = self.pat[idx_last].0;
        //println!("Pattern: {:?} :: Period {}", self.pat, t_period);

        // If we're not repeating, and we're off the end of the pattern
        // return the final value
        if (self.repeat < 0.0) && (t > (self.t_delay + t_period)) {
            return self.pat[idx_last].1;
        }

        // where in the cycle are we?
        let t_cycle = ( t - self.t_delay ) % t_period;
        if t_cycle < 0.0 {
            return 0.0;
        }

        // find out which points we're between...
        let mut t1 = 0.0;
        let mut t2 = 0.0;
        let mut v1 = 0.0;
        let mut v2 = 0.0;
        for tv in &self.pat {
            //println!("time: {}; tv: {:?}", t_cycle, tv);
            if t_cycle > tv.0 {
                t1 = tv.0;
                v1 = tv.1;
            } else {
                t2 = tv.0;
                v2 = tv.1;
                break;
            }
        }

        //dbg!(t1,v1, t2,v2);

        // ... and interpolate between them
        let mut gradient = (v2-v1) / (t2-t1);
        if gradient.is_nan() {
            gradient = 0.0;
        }
        gradient * (t_cycle - t1) + v1

    }

}
    
