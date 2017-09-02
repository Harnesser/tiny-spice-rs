use circuit::{NodeId, BOLTZMANN, CHARGE};
use newton_raphson::{Differentiable};

pub struct Diode {
    pub i_sat: f32,
    pub tdegc: f32,
}

impl Differentiable for Diode {

    fn eval(&self, v_d: f32) -> f32 {
        let v_t = BOLTZMANN * (363.0 + self.tdegc) / CHARGE;
        self.i_sat * (v_d / v_t).exp()
    }

    fn slope(&self, v_d: f32) -> f32 {
        let v_t = BOLTZMANN * (363.0 + self.tdegc) / CHARGE;
        (1.0/v_t) * self.i_sat * (v_d / v_t).exp()
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diode_current() {
        let d1 = Diode {
            i_sat: 1e-9,
            tdegc: 27.0,
        };
        let i_d = d1.eval(0.6);
        assert!( i_d == 0.0, "i_d was {}", i_d);
    }

    #[test]
    fn test_diode_slope() {
        let d1 = Diode {
            i_sat: 10e-9,
            tdegc: 27.0,
        };
        let i_slope = d1.slope(0.6);
        assert!( i_slope == 0.0, "i_slope was {}", i_slope );
    }

}
