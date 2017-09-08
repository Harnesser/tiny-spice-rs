
use circuit;

pub fn banner() {

    println!("********************************************");
    println!("***       Tiny-SPICE-Simulator           ***");
    println!("***        (c) CrapCorp 2017             ***");
    println!("*** Patent Pending, All rights reserved  ***");
    println!("********************************************");

}

pub struct Engine {
    // Number of voltage nodes in the circuit
    c_nodes: usize,

    // Number of voltage sources in the circuit
    // we have to solve for the current through these too
    c_vsrcs: usize,

    // base matrix - all the linear things
    base_matrix: Vec<Vec<f32>>,

    // list of non-linear elements in the circuit
    nonlinear_elements: Vec<circuit::Element>,

}

impl Engine {

    pub fn new() -> Engine {
        Engine {
            c_nodes: 0,
            c_vsrcs: 0,
            base_matrix: vec![vec![]],
            nonlinear_elements: vec![],
        }
    }

    pub fn dc_operating_point(&mut self, ckt: &circuit::Circuit) -> Vec<f32> {

        const RELTOL: f32 = 0.0001;
        const VNTOL: f32 = 1.0e-6;
        const ABSTOL: f32 = 1.0e-9;
        const ITL1: usize = 100;

        // build the circuit matrix
        self.elaborate(&ckt);
        
        // prep values for convergence checks
        let c_mna = self.c_nodes + self.c_vsrcs;
        let mut unknowns_prev : Vec<f32> = vec![0.0; c_mna];
        let mut unknowns : Vec<f32> = vec![];

        // Newton-Raphson loop
        let mut converged = false;
        let mut c_iteration: usize = 0;
        while c_iteration < ITL1 {

            // copy the base matrix, cos we're going to change it a lot:
            // * stamp non-linear element companion models
            // * re-order during guassian elimination
            let mut v = self.base_matrix.clone();

            // stamp companion models of non-linear devices
            self.nonlinear_stamp(&mut v, &unknowns_prev);

            // Guassian elimination & back solve of the now linearized
            // circuit matrix
            unknowns = self.solve(v);


            // Convergence check
            println!("*INFO* Convergence check");
            println!("{:?}", unknowns);
            println!("{:?}", unknowns_prev);

            if c_iteration > 0 {
                converged = true;
                for (i,x) in unknowns.iter().enumerate() {
                    let mut limit: f32 = 0.0;
                    if i >= self.c_nodes {
                        limit = x.abs() * RELTOL + VNTOL;
                    } else {
                        limit = x.abs() * RELTOL + ABSTOL;
                    }
                    let this = (x - unknowns_prev[i]).abs();
                    println!(" {} < {} ({})", this, limit, converged);
                    if this > limit {
                        converged = false;
                        // break;
                    }
                }
            }

            if converged {
                break;
            }

            //
            unknowns_prev = unknowns.clone();
            c_iteration += 1;
        }


        if converged {
            println!("*INFO* Converged after {} iterations", c_iteration + 1);
        } else {
            println!("*ERROR* Divergent");
        }

        unknowns
    }


    // Look at the circuit, and initialise linear version of the matrix
    fn elaborate(&mut self, ckt: &circuit::Circuit) {
        // assume here that nodes have been indexed 0 -> N-1
        // where n is the number of nodes (including ground) in the circuit

        // Number of nodes, including ground (aka 0, aka gnd)
        self.c_nodes = ckt.count_nodes();
        println!("*INFO* There are {} nodes in the design, including ground", self.c_nodes);

        // Number of voltage sources in the design
        self.c_vsrcs = ckt.count_voltage_sources();
        println!("*INFO* There are {} voltage sources in the design", self.c_vsrcs);

        println!("\n*INFO* Building Voltage Node Matrix and Current Vector");

        // Modified Nodal Analysis (MNA) Matrix
        let c_mna = self.c_nodes + self.c_vsrcs;
        // I think I have to make this out of Vecs (on the heap) because c_nodes is
        // not known at compile time. Makes sense, I suppose - could blow the stack if
        // c_nodes is any way huge.
        // [ V I ]
        self.base_matrix = vec![ vec![0.0; c_mna+1]; c_mna]; // +1 for currents
        let ia = c_mna; // index for ampere vector

        // Fill up the voltage node and current vector
        // This needs to know about each of the kinds of circuit elements, so
        // the node equations can be built up appropriately.
        let mut i_vsrc : usize = self.c_nodes; // index, not amperage...
        #[allow(unused_parens)]
        for el in &ckt.elements {
            match *el {
                // From NGSPICE manual:
                // Positive current is assumed to flow from the positive node,
                // through the source, to the negative node.
                // A current source of positive value forces current to flow 
                // out of the n+ node, through the source, and into the n- node.
                circuit::Element::I(circuit::CurrentSource{ ref p, ref n, ref value }) => {
                    println!("  [ELEMENT] Current source: {}A into node {} and out of node {}",
                            value, p, n);
                    if *p != 0 {
                        self.base_matrix[*p][ia] = self.base_matrix[*p][ia] - value;
                    }
                    if *n != 0 {
                        self.base_matrix[*n][ia] = self.base_matrix[*n][ia] + value;
                    }
                }
                circuit::Element::R(circuit::Resistor{ ref a, ref b, ref value }) => {
                    println!("  [ELEMENT] Resistor");
                    let over = 1.0 / value;

                    // out of node 'a'
                    if *a != 0 {
                        self.base_matrix[*a][*a] = self.base_matrix[*a][*a] + over;
                        if *b != 0 {
                            self.base_matrix[*a][*b] = self.base_matrix[*a][*b] - over;
                        }
                    }

                    // out of node 'b'
                    if *b != 0 {
                        self.base_matrix[*b][*b] = self.base_matrix[*b][*b] + over;
                        if *a != 0 {
                            self.base_matrix[*b][*a] = self.base_matrix[*b][*a] - over;
                        }
                    }
                }
                circuit::Element::V(circuit::VoltageSource{ ref p, ref n, ref value }) => {
                    println!("  [ELEMENT] Voltage source: {}V from node {} to node {}",
                            value, p, n);

                    // put the voltage value in the 'known' vector
                    self.base_matrix[i_vsrc][ia] = *value;

                    let p_not_grounded = (*p != 0);
                    let n_not_grounded = (*n != 0);

                    if p_not_grounded {
                        self.base_matrix[i_vsrc][*p] = 1.0;
                        self.base_matrix[*p][i_vsrc] = 1.0;
                    }

                    if n_not_grounded {
                        self.base_matrix[i_vsrc][*n] = -1.0;
                        self.base_matrix[*n][i_vsrc] = -1.0;
                    }

                    i_vsrc += 1; // voltage source matrix index update 
                    
                }
                circuit::Element::D(circuit::Diode{ ref p, ref n, ref i_sat, ref tdegc }) => {
                    println!("  [ELEMENT] Diode:");
                    self.nonlinear_elements.push(
                        circuit::Element::D(
                            circuit::Diode {
                                p: *p,
                                n: *n,
                                i_sat: *i_sat,
                                tdegc: *tdegc,
                            }
                        )
                    );
                }
                
            }
        }
        self.pp_matrix(&self.base_matrix);

    }

    // Solve the system of linear equations
    fn solve(&self, mut v: Vec<Vec<f32>>) -> Vec<f32> {

        let c_mna = self.c_nodes + self.c_vsrcs;
        let ia = c_mna; // index for ampere vector

        // Gaussian elimination with partial pivoting
        // https://en.wikipedia.org/wiki/Gaussian_elimination#Pseudocode
        println!("\n*INFO* Gaussian Elimination");
        for r_ref in 1..c_mna-1 { // column we're eliminating, but index rows

            // find the k-th pivot
            let r_max = self.index_of_next_abs(&v, r_ref);

            // swap
            if v[r_max][r_ref] == 0.0 {
                println!("*ERROR* Matrix is singular! {}", v[r_max][r_ref]);
                break;
            }
            v.swap(r_max, r_ref);

            // check that we're not going to divide by zero
            if v[r_ref][r_ref] == 0.0 {
                println!("*INFO* Skipping v[{}][..]", r_ref);
                continue;
            }

            for r_mod in r_ref+1..c_mna { // row we're scaling
                if v[r_mod][r_ref] == 0.0 {
                    //println!("Skipping v[{}][{}]", r_mod, r_ref);
                    continue;
                }
                let ratio = v[r_mod][r_ref] / v[r_ref][r_ref];

                for c_mod in r_ref..c_mna+1 { // column we're scaling
                    let val = v[r_mod][c_mod];
                    let wiggle = v[r_ref][c_mod];
                    let new = val - (wiggle * ratio); 
                    v[r_mod][c_mod] = new;
                    //println!("\nr_ref = {}, r_mod = {}, c_mod = {}, ratio = {}",
                    //         r_ref, r_mod, c_mod, ratio);
                    //println!("{} - {}*{} -> {}", val, wiggle, ratio, new);
                    //self.pp_matrix(&v);
                }
                //println!(" ---------------------------------------------- ");
            }
        }
        self.pp_matrix(&v);
      
        // TODO check result



        println!("\n*INFO* Back-substitution");

        // node voltage array
        let mut n = vec![0.0; c_mna];

        // Solve easiest
        let i_last = c_mna - 1;
        n[i_last] = v[i_last][c_mna] / v[i_last][i_last];

        // Solve the rest recursively
        for i_solve in (1..c_mna-1).rev() {
            let mut sum = 0.0;
            for i_term in i_solve+1..c_mna {
                sum += v[i_solve][i_term] * n[i_term];
            }
            n[i_solve] = ( v[i_solve][ia] - sum ) / v[i_solve][i_solve];
        }


        println!("\n*INFO* Results");
        for i_res in 1..self.c_nodes {
            println!(" v[{:2}] = {}", i_res, n[i_res]);
        }
        for i_res in self.c_nodes..self.c_nodes+self.c_vsrcs {
            println!(" i[{:2}] = {}", i_res, n[i_res]);
        }

        n

    }

    fn index_of_next_abs( &self, m: &Vec<Vec<f32>>, k: usize ) -> usize {
        let mut biggest: f32 = 0.0;
        let mut r_biggest: usize = k;
        let c_rows = m.len();
        for r in k..c_rows {
            let this = m[r][k].abs();
            if this > biggest {
                biggest = this;
                r_biggest = r;
            }
        }
        r_biggest
    }

    fn pp_matrix( &self, m : &Vec<Vec<f32>> ) {
        for r in m {
            for val in r {
                print!("{:.3}   ", val);
            }
            println!("");
        }
    }


    // stamp a matrix with linearized companion models of all the non-linear
    // devices listed in the SPICE netlist
    fn nonlinear_stamp(&self, m: &mut Vec<Vec<f32>>, n: &Vec<f32> ) {
        println!("*INFO* Stamping non-linear elements");
        for el in &self.nonlinear_elements {
            match *el {
                circuit::Element::D(circuit::Diode{ ref p, ref n, ref i_sat, ref tdegc }) => {
                    println!("*INFO* {}", el);
                }

                _ => { println!("*ERROR* - unrecognised nonlinear element"); }
            }
        }

    }

}

