
use circuit;
use circuit::NodeId;

pub fn banner() {

    println!("********************************************");
    println!("***       Tiny-SPICE-Simulator           ***");
    println!("***        (c) CrapCorp 2017             ***");
    println!("*** Patent Pending, All rights reserved  ***");
    println!("********************************************");

}

pub enum ConvergenceError {
    Divergent,
}

pub type ConvergenceResult = Result<bool, ConvergenceError>;


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

    pub fn transient_analysis(&mut self, ckt: &circuit::Circuit) {

        // user-supplied control on the sim time
        const TSTART: f32 = 0.0;
        const TSTOP: f32 = 1e-3;
        const TSTEP: f32 = 5e-2;

        // Iteration limits

        // Initial timestep factor
        const FS: f32 = 0.25;

        // Timestamp adjustment factor on iteration failure
        const FT: f32 = 0.25;

        // Smallest delta-time step allowed
        const RMIN: f32 = 1e-9;

        // Largest delta-time step allowed factor
        const RMAX: f32 = 5.0;

        // 'Easy' iteration count limit
        // If we solve in fewer iterations, increase delta-time
        const ITL3: usize = 6;

        // 'Struggling' iteration count limit
        // If this count is reached before a solution is found, reduce the 
        // delta-time step and restart the solution attempt
        const ITL4: usize = 20;



        // build the circuit matrix
        self.elaborate(&ckt);

        // prep values
        let c_mna = self.c_nodes + self.c_vsrcs;
        let mut unknowns_prev : Vec<f32> = vec![0.0; c_mna];
        let mut unknowns : Vec<f32> = vec![];

        // Find the DC operating point
        // used as the initial values in the transient simulation
        unknowns = self.dc_operating_point(&ckt);
        println!("*INFO*: DC : {:?}", &unknowns);

        // transient loop
        let mut t_delta = TSTEP * FS;
        let mut t_now = 0.0;

        // announce
        println!("Transient analysis: {} to {} by {}", TSTART, TSTOP, t_delta);

        // timestep loop
        let mut error = false;
        let mut is_final_timestep = false;
        let mut c_step = 0;
        loop {

            // At the start of the loop, we've a candidate t_delta to solve on.
            // This comes from either:
            // * the initial calculation after DC on the initial iteration
            // * the prevous go round the loop for other iterations
            let t_try = t_now + t_delta;

            // stamp non-linear components, passing in the current time as
            // some ... dunno sinewaves will take t_try, but maybe caps need
            // t_delta? pass both?
            
            if t_now >= TSTART && t_now != 0.0 {
                println!("*INFO*: [{}] t={} : {:?}", c_step, t_now, unknowns);
            }

            if is_final_timestep {
                break;
            }

            // update things for next loop
            unknowns_prev = unknowns.to_vec();

            // solver loop
            // breaks when solved, or time-step too small

            // solver iteration count
            let mut c_iteration: usize = 0;
/*
            println!("*INFO* Time: {}", t_now);
            loop {
                println!("*INFO* Iteration {}", c_iteration);

                // Solve
                //if let Some(unknowns) = solve_trans(v, t_now, t_delta);

                // check if we're ok to continue iterating
                if c_iterations >= ITL4 {
                    t_delta = t_delta * FT;
                    if t_delta < TF {
                        println!("*ERROR* Timestep too small");
                        error = true;
                    } else {
                        // reset iteration count
                        c_iteration = 0;
                    }
                } else {
                    // update iteration counts
                    c_iteration += 1;
                }

            }

            if converged {
                // solver found it too easy, maybe there's not a lot going on
                // reduce the t_delta
                if c_iterations < ITL3 {
                    t_delta = t_delta * 2.0;
                    let t_delta_max = TSTEP * RMAX;
                    if t_delta > t_delta_max {
                        t_delta = t_delta_max;
                    }
                }
            }
            if !converged {
                }

            }

            t_now += t_delta;
            c_step += 1;
            if t_now > TSTOP {
                t_now = TSTOP;
                is_final_timestep = true;

            } // solver



            // break out of this loop if an error was detected
            if error {
                break;
            }
*/
        } // time

        println!("*INFO* Finished at time {}", t_now);

    }

    pub fn dc_operating_point(&mut self, ckt: &circuit::Circuit) -> Vec<f32> {

        const ITL1: usize = 50;

        // build the circuit matrix
        self.elaborate(&ckt);
        
        // prep values for convergence checks
        let c_mna = self.c_nodes + self.c_vsrcs;
        let mut unknowns_prev : Vec<f32> = vec![0.0; c_mna];
        let mut unknowns : Vec<f32> = vec![];

        let mut converged = false;

        // Newton-Raphson loop
        let mut c_iteration: usize = 0;
        while c_iteration < ITL1 {

            // copy the base matrix, cos we're going to change it a lot:
            // * stamp non-linear element companion models
            // * re-order during guassian elimination
            let mut v = self.base_matrix.clone();

            // stamp companion models of non-linear devices
            self.nonlinear_stamp(&mut v, &unknowns_prev);
            println!("*INFO* Non-linear stamped matrix");
            self.pp_matrix(&v);

            // Guassian elimination & back solve of the now linearized
            // circuit matrix
            unknowns = self.solve(v);


            // Convergence check
            println!("*INFO* Convergence check {}", c_iteration);
            println!("{:?}", unknowns);
            println!("{:?}", unknowns_prev);

            if c_iteration > 0 {
                match self.convergence_check(&unknowns, &unknowns_prev) {
                    Ok(cnvd) => {
                        if cnvd {
                            converged = true;
                            break;
                        }
                    },
                    Err(_) => {
                        println!("*ERROR* math gone bad");
                        break;
                    },
                }
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
        let mut m = vec![ vec![0.0; c_mna+1]; c_mna]; // +1 for currents

        // Fill up the voltage node and current vector
        // This needs to know about each of the kinds of circuit elements, so
        // the node equations can be built up appropriately.
        let mut i_vsrc : usize = self.c_nodes; // index, not amperage...
        for el in &ckt.elements {
            match *el {
                // From NGSPICE manual:
                // Positive current is assumed to flow from the positive node,
                // through the source, to the negative node.
                // A current source of positive value forces current to flow 
                // out of the n+ node, through the source, and into the n- node.
                circuit::Element::I(ref isrc) => {
                    self.stamp_current_source(&mut m, isrc);
                }

                circuit::Element::R(ref r) => {
                    self.stamp_resistor(&mut m, r);
                }

                circuit::Element::V(ref vsrc) => {
                    self.stamp_voltage_source(&mut m, vsrc, i_vsrc);
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
        self.base_matrix = m.to_vec();
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
                    println!("\nr_ref = {}, r_mod = {}, c_mod = {}, ratio = {}",
                             r_ref, r_mod, c_mod, ratio);
                    println!("{} - {}*{} -> {}", val, wiggle, ratio, new);
                    self.pp_matrix(&v);
                }
                //println!(" ---------------------------------------------- ");
            }
        }
        println!("\n*INFO* Final Matrix");
        self.pp_matrix(&v);
      
        // TODO check result



        println!("\n*INFO* Back-substitution");

        // node voltage array
        let mut n = vec![0.0; c_mna];

        // Solve easiest
        let i_last = c_mna - 1;
        n[i_last] = v[i_last][c_mna] / v[i_last][i_last];
        //println!("[lst]  {} / {}",  v[i_last][c_mna], v[i_last][i_last] );
        if !n[i_last].is_finite() {
            println!("*WARNING* have to hack the first solve to 0.0");
            println!(" This can happen if solving a 0V source from a node to ground");
            n[i_last] = 0.0;
        }

        // Solve the rest recursively
        for i_solve in (1..c_mna-1).rev() {
            let mut sum = 0.0;
            for i_term in i_solve+1..c_mna {
                sum += v[i_solve][i_term] * n[i_term];
                //println!("[{:3}]  {} * {}",  i_solve, v[i_solve][i_term], n[i_term]);

            }
            n[i_solve] = ( v[i_solve][ia] - sum ) / v[i_solve][i_solve];
            //println!("*INFO* {} - {} / {} = {}", 
            //        v[i_solve][ia], sum,
            //        v[i_solve][i_solve],
            //        n[i_solve]
            //);
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

    fn pp_matrix(&self, m : &Vec<Vec<f32>> ) {
        for r in m {
            for val in r {
                print!("{:.3}   ", val);
            }
            println!("");
        }
    }

    fn stamp_current_source(&self, m: &mut Vec<Vec<f32>>, isrc: &circuit::CurrentSource) {
        println!("  [ELEMENT] Current source: {}A into node {} and out of node {}",
                isrc.value, isrc.p, isrc.n);
        let ia = self.c_nodes + self.c_vsrcs; // index for ampere vector
        if isrc.p != 0 {
            m[isrc.p][ia] = m[isrc.p][ia] - isrc.value;
        }
        if isrc.n != 0 {
            m[isrc.n][ia] = m[isrc.n][ia] + isrc.value;
        }
    }

    #[allow(unused_parens)]
    fn stamp_voltage_source(
        &self,
        m: &mut Vec<Vec<f32>>,
        vsrc: &circuit::VoltageSource,
        i_vsrc: NodeId,
    ) {
        println!("  [ELEMENT] Voltage source: {}V from node {} to node {}",
                vsrc.value, vsrc.p, vsrc.n);
        let ia = self.c_nodes + self.c_vsrcs; // index for ampere vector

        // put the voltage value in the 'known' vector
        m[i_vsrc][ia] = vsrc.value;

        let p_not_grounded = (vsrc.p != 0);
        let n_not_grounded = (vsrc.n != 0);

        if p_not_grounded {
            m[i_vsrc][vsrc.p] = 1.0;
            m[vsrc.p][i_vsrc] = 1.0;
        }

        if n_not_grounded {
            m[i_vsrc][vsrc.n] = -1.0;
            m[vsrc.n][i_vsrc] = -1.0;
        }
    }



    fn stamp_resistor(&self, m: &mut Vec<Vec<f32>>, r: &circuit::Resistor) {
        println!("  [ELEMENT] Resistor {} Ohms between node {} and node {}",
                r.value, r.a, r.b);
        let over = 1.0 / r.value;

        // out of node 'a'
        if r.a != 0 {
            m[r.a][r.a] = m[r.a][r.a] + over;
            if r.b != 0 {
                m[r.a][r.b] = m[r.a][r.b] - over;
            }
        }

        // out of node 'b'
        if r.b != 0 {
            m[r.b][r.b] = m[r.b][r.b] + over;
            if r.a != 0 {
                m[r.b][r.a] = m[r.b][r.a] - over;
            }
        }
    }

    // stamp a matrix with linearized companion models of all the non-linear
    // devices listed in the SPICE netlist
    fn nonlinear_stamp(&self, m: &mut Vec<Vec<f32>>, n: &Vec<f32> ) {
        println!("*INFO* Stamping non-linear elements");
        for el in &self.nonlinear_elements {
            match *el {
                circuit::Element::D(ref d) => {
                    println!("*INFO* {}", el);

                    // linearize
                    let v_d = n[d.p] - n[d.n];
                    let (g_eq, i_eq) = d.linearize(v_d);

                    // stamp
                    self.stamp_current_source(m, &circuit::CurrentSource{
                        p: d.p,
                        n: d.n,
                        value: i_eq
                    });
                    self.stamp_resistor(m, &circuit::Resistor{
                        a: d.p,
                        b: d.n,
                        value: 1.0/g_eq
                    });
                }

                _ => { println!("*ERROR* - unrecognised nonlinear element"); }
            }
        }
    }


    // check for convergence by testing new and previous solutions against
    // RELTOL and the like
    pub fn convergence_check(&self, xv: &Vec<f32>, yv: &Vec<f32>) -> ConvergenceResult {

        // 
        const RELTOL: f32 = 0.0001;
        const VNTOL: f32 = 1.0e-6;
        const ABSTOL: f32 = 1.0e-9;

        let mut res = Ok(true);
        for (i,x) in xv.iter().enumerate() {
            if !x.is_finite() {
                println!("*ERROR* math gone bad");
                res = Err(ConvergenceError::Divergent);
                break;
            }
            let limit: f32;
            if i >= self.c_nodes {
                limit = x.abs() * RELTOL + VNTOL;
            } else {
                limit = x.abs() * RELTOL + ABSTOL;
            }
            let this = (x - yv[i]).abs();
            println!(" {} < {}", this, limit);
            if this > limit {
                res = Ok(false);
            }
        }
        res
    }

}

