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
            m: vec![ vec![0.0; n_node+n_vsrc]; n_node+n_vsrc],
        }
    }

}
