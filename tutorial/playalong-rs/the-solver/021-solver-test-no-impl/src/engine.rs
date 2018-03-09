//! SPICE circuit simulator engine
//!
//! Datastructure to manage the circuit matrix of the simulator

/// Simulator engine datastructure
pub struct Engine {

    /// Number of nodes in the circuit
    pub n_node: usize,

    /// Number of independent voltage sources
    pub n_vsrc: usize,

    /// Circuit Matrix
    pub m: Vec<Vec<f32>>,

}

impl Engine {

    /// Initialise the `Engine` structure.
    ///
    /// We need to know the final node and voltage source count
    /// before calling this function
    pub fn new(n_node: usize, n_vsrc: usize) -> Engine {
        println!("*INFO* Initialising circuit matrix");
        Engine {
            n_node: n_node,
            n_vsrc: n_vsrc,
            m: vec![ vec![0.0; n_node+n_vsrc+1]; n_node+n_vsrc],
        }
    }

    /// Solve the system of linear equations represented by the matrix
    ///
    /// Using Guassian Elimination: row reordering, then back-substitution
    pub fn solve(&mut self) -> Result< Vec<f32>, &'static str> {
        Ok( vec![0.0; self.n_vsrc + self.n_node] )
    }
}


#[cfg(test)]
mod tests {

    use super::*; // magic so we can use `Engine`

    #[test]
    fn it_works() {

        let mut eng = Engine::new(3,0);

        // fill the matrix with the example from wikipedia:
        // https://en.wikipedia.org/wiki/Gaussian_elimination#Example_of_the_algorithm
        eng.m[0][0] = 2.0;
        eng.m[0][1] = 1.0;
        eng.m[0][2] = -1.0;
        eng.m[0][3] = 8.0; // augmented column

        eng.m[1][0] = -3.0;
        eng.m[1][1] = -1.0;
        eng.m[1][2] = 2.0;
        eng.m[1][3] = -11.0; // augmented column

        eng.m[2][0] = -2.0;
        eng.m[2][1] = 1.0;
        eng.m[2][2] = 2.0;
        eng.m[2][3] = 2.0; // augmented column

        println!("{:?}", eng.m);

        let res = eng.solve();

        assert_eq!(res, Ok(vec![2.0, 3.0, 1.0]) );
    }

}

