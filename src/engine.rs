//! SPICE Ionsamhlóir Ciorcaid
//!
//! Contains the stamper, solver and convergence checkers

use crate::analysis;
use crate::circuit;
use crate::wavewriter::WaveWriter;

/// Program execution trace macro - prefix `<engine>`
macro_rules! trace {
    ($fmt:expr $(, $($arg:tt)*)?) => {
        // uncomment the line below for tracing prints
        //println!(concat!("<engine> ", $fmt), $($($arg)*)?);
    };
}

fn banner() {

    println!("**********************************************");
    println!("***          Tiny-SPICE-Simulator          ***");
    println!("***       (c) CrapCadCorp 2017-2022        ***");
    println!("*** No Patents Pending, No rights reserved ***");
    println!("**********************************************");

}

#[derive(Debug)]
pub enum ConvergenceError {
    Divergent,
}

pub type ConvergenceResult = Result<bool, ConvergenceError>;


#[derive(Default)]
#[allow(non_snake_case)]
pub struct Engine {

    // Number of voltage nodes in the circuit
    c_nodes: usize,

    // Number of voltage sources in the circuit
    // we have to solve for the current through these too
    c_vsrcs: usize,

    // base matrix - all the linear things
    base_matrix: Vec<Vec<f64>>,

    // list of nonlinear elements in the circuit
    nonlinear_elements: Vec<circuit::Element>,

    // list of independent sources
    independent_sources: Vec<circuit::Element>,

    // list of voltage-dependent sources
    v_dependent_sources: Vec<circuit::Element>,

    // list of elements with energy storage (caps & inductors)
    storage_elements: Vec<circuit::Element>,

    // DC operating point
    dc_op: Vec<f64>,

}

impl Engine {

    pub fn new() -> Engine {
        banner();
        Engine {
            c_nodes: 0,
            c_vsrcs: 0,
            base_matrix: vec![vec![]],
            nonlinear_elements: vec![],
            independent_sources: vec![],
            v_dependent_sources: vec![],
            storage_elements: vec![],
            dc_op: vec![],
        }
    }

    /// Run the selected circuit analysis
    ///
    /// This dispatches out to the different analysis routines (e.g. DC operating
    /// point or transient) depending on the configuration.
    pub fn go(
        &mut self,
        ckt: &circuit::Circuit,
        cfg: &analysis::Configuration,
    ) -> Option<analysis::Statistics> {
        if let Some(ref a) = cfg.kind {
            match *a {
                analysis::Kind::DcOperatingPoint => Some(self.dc_operating_point(ckt, cfg)),
                analysis::Kind::Transient => Some(self.transient_analysis(ckt, cfg)),
                _ => {
                    println!("*ERROR* unsupported circuit analysis type");
                    None
                }
            }
        } else {
            println!("*ERROR* analysis type is not set");
            None
        }
    }

    // Grab the DC operating point values
    pub fn dc(self) -> Option<Vec<f64>> {
        if self.dc_op.is_empty() {
            None
        } else {
            Some(self.dc_op.clone())
        }
    }

/*
    // need to know which element to sweep
    pub fn dc_sweep(
        &mut self,
        ckt: &circuit::Circuit,
        cfg: &analysis::Configuration,
    ) {
        const VSTART: f64 = -3.0;
        const VSTOP: f64 = 5.0;
        const VSTEPS: usize = 100;

        let v_step = (VSTOP - VSTART) / VSTEPS as f64;

        self.elaborate(&ckt);

        // announce
        println!("*************************************************************");
        println!("DC Sweep: {} to {} by {}", VSTART, VSTOP, v_step);

        // open waveform database
        let mut wavedb = WaveWriter::new(&cfg.wavefile).unwrap();
        wavedb.header(self.c_nodes, self.c_vsrcs);

        // FUCK!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        // have to stamp this all the time
        // FIXCasdfkj disf=
        //0]
        //

        // FIXME very fragile - what if there's more than one voltage source in
        // the design?
        let idx_vsrc : usize = self.c_nodes; // index, not amperage...

        // tweak the thing we're sweeping
        for s in 0..VSTEPS {
            let v_sweep = VSTART + (v_step * s as f64);
            //let self.

            let mut mna = self.base_matrix.clone();

            // unstamp the voltage source
            // FIXME - assume it's 0V
            let v_src = circuit::VoltageSource{
                p: 1,
                n: 0,
                value: v_sweep,
            };

            self.stamp_voltage_source(&mut mna, &v_src, idx_vsrc);
            
            let _stats = self.dc_solve(&mna, &cfg);
            wavedb.dump_vector(v_sweep, &self.dc_op);

        }

    }
*/

    pub fn transient_analysis(
        &mut self,
        ckt: &circuit::Circuit,
        cfg: &analysis::Configuration,
    ) 
    -> analysis::Statistics
    {

        // Find the DC operating point
        // used as the initial values in the transient simulation
        // this will also build the circuit
        let dc_op_stats = self.dc_operating_point(ckt, cfg);
        let mut unknowns = self.dc_op.clone();

        trace!(" [TRANSIENT] : DC : {:?}", &unknowns);

        // prep values
        let c_mna = self.c_nodes + self.c_vsrcs;
        let mut unknowns_prev : Vec<f64> = vec![0.0; c_mna];

        // transient loop
        let mut t_delta = cfg.TSTEP * cfg.FS;
        let t_delta_min = cfg.TSTEP * cfg.RMIN; // not mimimum resistance...
        let mut t_now = 0.0;

        // announce
        println!("*************************************************************");
        println!("*CONFIG* TRANSIENT ANALYSIS");
        println!("*CONFIG* TIME {} to {} by {:0.12}",
                 cfg.TSTART, cfg.TSTOP, cfg.TSTEP);
        println!("*************************************************************");

        // open waveform database
        let mut wavedb = WaveWriter::new(&cfg.wavefile, &ckt.node_id_lut).unwrap();
        wavedb.header(self.c_nodes, self.c_vsrcs);
        wavedb.dump_vector(t_now, &unknowns); // DC solution

        // timestep loop
        let mut is_final_timestep = false;
        #[allow(unused_variables)]
        let mut c_step = 0; // goes away with empty trace!
        let mut c_iteration: usize = 0;
        loop {

            // At the start of the loop, we've a candidate t_delta to solve on.
            // This comes from either:
            // * the initial calculation after DC on the initial iteration
            // * the prevous go round the loop for other iterations

            if t_now >= cfg.TSTART && t_now != 0.0 {
                trace!("*DATA*: [{}] t={} : {:?}", c_step, t_now, unknowns);
                wavedb.dump_vector(t_now, &unknowns);
            }

            if is_final_timestep {
                break;
            }


            // solver loop
            // breaks when solved, or time-step too small

            let mut converged = false;
            let mut error = false;

            // solver iteration count
            let mut c_itl: usize = 0;
            let mut unknowns_solve : Vec<f64> = vec![0.0; c_mna];
            let mut unknowns_solve_prev : Vec<f64> = vec![0.0; c_mna];
            let mut geared = false;

            let mut _mse :f64 = 0.0; // not used if trace! is empty

            loop {

                trace!("*METRIC* {} {} {} {} {} {}",
                         c_step, t_now, t_delta, c_iteration, c_itl, _mse);

                // copy the base matrix, cos we're going to change it a lot:
                // * stamp nonlinear element companion models
                // * re-order during guassian elimination
                let mut m = self.base_matrix.clone();

                // stamp independent sources
                self.independent_source_stamp(&mut m, t_now + t_delta);

                // Stamp voltage-dependent sources
                self.v_dependent_source_stamp(&mut m);

                // stamp elements that store energy
                self.storage_stamp(&mut m, &unknowns_prev, t_delta);

                // stamp companion models of nonlinear devices
                self.nonlinear_stamp(&mut m, &unknowns, &unknowns_solve_prev);

                // Solve
                unknowns = self.solve(m);
                _mse = self.mean_squared_error(&unknowns_solve, &unknowns);

                // enable this to plot delta-time
                //wavedb.dump_vector(t_now, &unknowns);

                // update loop counters
                c_itl += 1;
                c_iteration += 1;

                // Convergence check
                match self.convergence_check(&unknowns, &unknowns_solve, cfg) {
                    Ok(cnvg) => {
                        if cnvg {
                            trace!(" [TIMESTEP] Timestep converged after {} iterations", c_itl);
                            converged = true;
                            break;
                        } else {
                            // adjust timestep if we can
                            if c_itl >= cfg.ITL4 {
                                t_delta *= cfg.FT;
                                // check if we're ok to continue iterating
                                if t_delta < t_delta_min {
                                    println!("*ERROR* Internal timestep too small");
                                    error = true;
                                    break;
                                } else {
                                    // reset iteration count
                                    trace!(" [TIMESTEP] Upshifting -> new t_delta = {}", t_delta);
                                    geared = true;
                                    c_itl = 0;
                                }
                            }
                        }
                    },
                    Err(_) => {
                        println!("*ERROR* There was a numerical error");
                        error = true;
                        break;
                    },
                }
                unknowns_solve_prev = unknowns_solve.to_vec();
                unknowns_solve = unknowns.to_vec();
            }

            if converged {
                // update things for next loop
                unknowns_prev = unknowns.to_vec();

                // solver found it too easy, maybe there's not a lot going on
                // reduce the t_delta
                if !geared & (c_itl < cfg.ITL3) {
                    t_delta *= 2.0;
                    let t_delta_max = cfg.TSTEP * cfg.RMAX;
                    if t_delta > t_delta_max {
                        trace!(" [TIMESTEP] Downshifting maxed out");
                        t_delta = t_delta_max;
                    } else {
                        trace!(" [TIMESTEP] Downshifting -> new t_delta = {}", t_delta);
                    }
                }
            }

            c_step += 1;
            t_now += t_delta;
            if t_now > cfg.TSTOP {
                t_now = cfg.TSTOP;
                is_final_timestep = true;

            } // solver

            // break out of this loop if an error was detected
            if error {
                println!("*ERROR* bad stuff happened, breaking out of timestep loop");
                break;
            }
        } // time

        println!("*INFO* Finished at time {}", t_now);
        analysis::Statistics {
            kind: analysis::Kind::Transient,
            end: t_now,
            iterations: dc_op_stats.iterations + c_iteration,
        }
    }


    // assume circuit has been elaborated
    fn dc_solve(
        &mut self,
        mna: &[Vec<f64>],
        cfg: &analysis::Configuration,
    )
        -> analysis::Statistics
    {

        // prep values for convergence checks
        let c_mna = self.c_nodes + self.c_vsrcs;
        let mut unknowns_prev : Vec<f64> = vec![0.0; c_mna];
        let mut unknowns_prev_prev : Vec<f64> = vec![0.0; c_mna];
        let mut unknowns : Vec<f64> = vec![];

        let mut converged = false;

        // Newton-Raphson loop
        let mut c_iteration: usize = 0;

        while c_iteration < (cfg.ITL1+1) {
            c_iteration += 1;

            // copy the base matrix, cos we're going to change it a lot:
            // * stamp nonlinear element companion models
            // * re-order during guassian elimination
            let mut m = mna.to_owned();

            // Stamp independent sources at time=0.0
            // !!!FIXME!!! - hoist out of loop?
            self.independent_source_stamp(&mut m, 0.0);

            // Stamp voltage-dependent sources
            self.v_dependent_source_stamp(&mut m);

            // stamp companion models of nonlinear devices
            self.nonlinear_stamp(&mut m, &unknowns_prev, &unknowns_prev_prev);

            // Guassian elimination & back solve of the now linearized
            // circuit matrix
            unknowns = self.solve(m);

            // Convergence check
            trace!(" [CONVERGE] Convergence check {}", c_iteration);
            trace!(" [CONVERGE]  {:?}", unknowns);
            trace!(" [CONVERGE]  {:?}", unknowns_prev);

            if c_iteration > 0 {
                match self.convergence_check(&unknowns, &unknowns_prev, cfg) {
                    Ok(cnvd) => {
                        if cnvd {
                            converged = true;
                            break;
                        }
                    },
                    Err(_) => {
                        println!("*ERROR* math gone bad during DC solve");
                        break;
                    },
                }
            }

            // leave
            unknowns_prev_prev = unknowns_prev.clone();
            unknowns_prev = unknowns.clone();
        }


        if converged {
            trace!(" [CONVERGE] Converged after {} iterations", c_iteration);
        } else {
            println!("*ERROR* Divergent");
        }

        let stats = analysis::Statistics {
            kind: analysis::Kind::DcOperatingPoint,
            end: 0.0,
            iterations: c_iteration,
        };

        self.dc_op = unknowns.clone();
        stats
    }



    pub fn dc_operating_point(
        &mut self,
        ckt: &circuit::Circuit,
        cfg: &analysis::Configuration,
    )
        -> analysis::Statistics
    {

        // build the circuit matrix
        self.elaborate(ckt);

        // cos borrowck
        let mna = self.base_matrix.clone();
        self.dc_solve(&mna, cfg)
    }


    /// Look at the circuit, and initialise linear version of the matrix
    fn elaborate(&mut self, ckt: &circuit::Circuit) {
        // assume here that nodes have been indexed 0 -> N-1
        // where n is the number of nodes (including ground) in the circuit

        // Number of nodes, including ground (aka 0, aka gnd)
        self.c_nodes = ckt.count_nodes();
        println!("*INFO* There are {} nodes in the design, including ground", self.c_nodes);

        // Number of voltage sources in the design
        self.c_vsrcs = ckt.count_voltage_sources();
        println!("*INFO* There are {} voltage sources in the design", self.c_vsrcs);

        trace!("Building Voltage Node Matrix and Current Vector");

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
                    self.stamp_voltage_source(&mut m, vsrc);
                }

                circuit::Element::D(ref d) => {
                    trace!("  [ELEMENT] Diode:");
                    self.nonlinear_elements.push(
                        circuit::Element::D(d.clone())
                    );
                }

                circuit::Element::Isin(ref isrcsine) => {
                    trace!("  [ELEMENT] Current Source (~):");
                    self.independent_sources.push(
                        circuit::Element::Isin(isrcsine.clone())
                    );
                }

                circuit::Element::Vsin(ref vsrcsine) => {
                    trace!("  [ELEMENT] Voltage Source (~):");
                    self.independent_sources.push(
                        circuit::Element::Vsin(vsrcsine.clone())
                    );
                }

                circuit::Element::C(ref c) => {
                    trace!("  [ELEMENT] Capacitor:");
                    self.storage_elements.push(
                        circuit::Element::C(c.clone())
                    );
                }

                circuit::Element::Vpwl(ref vpwl) => {
                    trace!("  [ELEMENT] PWL Voltage Source:");
                    self.independent_sources.push(
                        circuit::Element::Vpwl(vpwl.clone())
                    );
                }

                circuit::Element::Vcvs(ref vcvs) => {
                    trace!("  [ELEMENT] VCVS:");
                    self.v_dependent_sources.push(
                        circuit::Element::Vcvs(vcvs.clone())
                    );
                }

                circuit::Element::Vccs(ref vccs) => {
                    trace!("  [ELEMENT] VCCS:");
                    self.v_dependent_sources.push(
                        circuit::Element::Vccs(vccs.clone())
                    );
                }
                
            }
        }
        self.base_matrix = m.to_vec();
        self.pp_matrix(&self.base_matrix);

    }

    // Solve the system of linear equations
    fn solve(&self, mut v: Vec<Vec<f64>>) -> Vec<f64> {

        let c_mna = self.c_nodes + self.c_vsrcs;
        let ia = c_mna; // index for ampere vector

        // Gaussian elimination with partial pivoting
        // https://en.wikipedia.org/wiki/Gaussian_elimination#Pseudocode
        trace!("*INFO* Gaussian Elimination");
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

                for c_mod in r_ref..=c_mna { // column we're scaling
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
        //println!("\n*INFO* Final Matrix");
        //self.pp_matrix(&v);
      
        // TODO check result

        trace!("*INFO* Back-substitution");

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
            #[allow(clippy::needless_range_loop)]
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


        trace!(" [SOLVE] Results");
        #[allow(clippy::needless_range_loop, unused_variables)]
        for i_res in 1..self.c_nodes {
            trace!(" v[{:2}] = {}", i_res, n[i_res]);
        }

        #[allow(clippy::needless_range_loop, unused_variables)]
        for i_res in self.c_nodes..self.c_nodes+self.c_vsrcs {
            trace!(" i[{:2}] = {}", i_res, n[i_res]);
        }

        n

    }

    fn index_of_next_abs( &self, m: &[Vec<f64>], k: usize ) -> usize {
        let mut biggest: f64 = 0.0;
        let mut r_biggest: usize = k;
        let c_rows = m.len();
        #[allow(clippy::needless_range_loop)]
        for r in k..c_rows {
            let this = m[r][k].abs();
            if this > biggest {
                biggest = this;
                r_biggest = r;
            }
        }
        r_biggest
    }

    // mean squared error of the two vectors
    fn mean_squared_error(&self, v1: &[f64], v2: &[f64]) -> f64 {
        let mut mse :f64 = 0.0;
        let bits = v1.iter().zip(v2.iter());
        for (x,y) in bits {
            mse += ( x - y ).powi(2);
        }
        mse /= v1.len() as f64;
        mse
    }


    fn pp_matrix(&self, m : &[Vec<f64>] ) {
        for r in m {
            for val in r {
                print!("{:.3}   ", val);
            }
            println!();
        }
    }


    fn stamp_current_source(&self, m: &mut [Vec<f64>], isrc: &circuit::CurrentSource) {
        trace!("  [STAMP] Current source: {}A into node {} and out of node {}",
                isrc.value, isrc.p, isrc.n);
        let ia = self.c_nodes + self.c_vsrcs; // index for ampere vector
        if isrc.p != 0 {
            m[isrc.p][ia] -= isrc.value;
        }
        if isrc.n != 0 {
            m[isrc.n][ia] += isrc.value;
        }
    }


    #[allow(unused_parens)]
    fn stamp_voltage_source(
        &self,
        m: &mut [Vec<f64>],
        vsrc: &circuit::VoltageSource,
    ) {
        trace!("  [STAMP] Voltage source: {}V from node {} to node {}",
                vsrc.value, vsrc.p, vsrc.n);
        let idx_vsrc = self.c_nodes + vsrc.idx; // index in ampere vector

        // put the voltage value in the 'known' vector
        let idx_unknown = m[0].len() - 1;
        m[idx_vsrc][idx_unknown] = vsrc.value;

        let p_not_grounded = (vsrc.p != 0);
        let n_not_grounded = (vsrc.n != 0);

        if p_not_grounded {
            m[idx_vsrc][vsrc.p] = 1.0;
            m[vsrc.p][idx_vsrc] = 1.0;
        }

        if n_not_grounded {
            m[idx_vsrc][vsrc.n] = -1.0;
            m[vsrc.n][idx_vsrc] = -1.0;
        }
    }



    fn stamp_resistor(&self, m: &mut [Vec<f64>], r: &circuit::Resistor) {
        trace!("  [STAMP] Resistor {} Ohms between node {} and node {}",
                r.value, r.a, r.b);
        let over = 1.0 / r.value;

        // out of node 'a'
        if r.a != 0 {
            m[r.a][r.a] += over;
            if r.b != 0 {
                m[r.a][r.b] -= over;
            }
        }

        // out of node 'b'
        if r.b != 0 {
            m[r.b][r.b] += over;
            if r.a != 0 {
                m[r.b][r.a] -= over;
            }
        }
    }


    fn storage_stamp(&self, m: &mut [Vec<f64>], n: &[f64], t: f64) {

        if !&self.storage_elements.is_empty() {
            trace!("  [STAMP] storage elements");
        }

        for el in &self.storage_elements {
            match *el {
                circuit::Element::C(ref c) => {

                    // linearize
                    let v_c = n[c.a] - n[c.b];
                    let (g_eq, i_eq) = c.linearize(v_c, t);

                    // stamp
                    self.stamp_current_source(m, &circuit::CurrentSource{
                        p: c.b,
                        n: c.a,
                        value: i_eq
                    });
                    self.stamp_resistor(m, &circuit::Resistor{
                        ident: "asdfa".to_string(),
                        a: c.a,
                        b: c.b,
                        value: 1.0/g_eq
                    });

                },
                _ => { println!("*ERROR* - unrecognised storage element"); }
            }
        }
        //println!("*INFO* Energy storage stamped matrix");
        //self.pp_matrix(&m);
    }


    // stamp a matrix with linearized companion models of all the nonlinear
    // devices listed in the SPICE netlist
    fn nonlinear_stamp(&self, m: &mut [Vec<f64>], n: &[f64], n_prev: &[f64] ) {

        if !&self.nonlinear_elements.is_empty() {
            trace!("  [STAMP] nonlinear elements");
        }

        for el in &self.nonlinear_elements {
            match *el {
                circuit::Element::D(ref d) => {

                    // linearize
                    let v_d = n[d.p] - n[d.n];
                    let v_d_prev = n_prev[d.p] - n_prev[d.n];
                    let (g_eq, i_eq) = d.linearize(v_d, v_d_prev);

                    trace!(" [STAMP] {} {} {:?}", el, v_d, (g_eq, i_eq));

                    // stamp
                    self.stamp_current_source(m, &circuit::CurrentSource{
                        p: d.p,
                        n: d.n,
                        value: i_eq
                    });
                    self.stamp_resistor(m, &circuit::Resistor{
                        ident: "nl_something".to_string(),
                        a: d.p,
                        b: d.n,
                        value: 1.0/g_eq
                    });
                }

                _ => { println!("*ERROR* - unrecognised nonlinear element"); }
            }
        }
        //println!("*INFO* Non-linear stamped matrix");
        //self.pp_matrix(&m);
    }

    fn v_dependent_source_stamp(&self, m: &mut [Vec<f64>]) {

        if !&self.v_dependent_sources.is_empty() {
            trace!("  [STAMP] voltage-dependent sources");
        }

        for el in &self.v_dependent_sources {
            match *el {

                circuit::Element::Vcvs(ref src) => {
                    let idx = self.c_nodes + src.idx; // index in ampere vector
                    trace!(" [STAMP] VCVS (idx:{} ({}))", src.idx, idx);

                    // branch current of output source
                    if src.p != 0 { m[src.p][idx] += 1.0 }
                    if src.n != 0 { m[src.n][idx] -= 1.0 }

                    // make sure controls and outputs are related
                    if src.cp != 0 { m[idx][src.cp] += src.k }
                    if src.cn != 0 { m[idx][src.cn] -= src.k }

                    if src.p != 0 { m[idx][src.p] -= 1.0 }
                    if src.n != 0 { m[idx][src.n] += 1.0 }
                }

                circuit::Element::Vccs(ref src) => {
                    // Reminder to self:
                    // +ve currents flow into a node
                    // -ve currents flow out of a node
                    // A VCCS with a +ve value:
                    //   -> (p) ----> (n) ->
                    // (in thru p, out thru n)
                    trace!(" [STAMP] VCCS");
                    if src.p != 0 {
                        if src.cp != 0 { m[src.p][src.cp] += src.k }
                        if src.cn != 0 { m[src.p][src.cn] -= src.k }
                    }
                    if src.n != 0 {
                        if src.cp != 0 { m[src.n][src.cp] -= src.k }
                        if src.cn != 0 { m[src.n][src.cn] += src.k }
                    }
                }

                _ => { println!("*ERROR* - unrecognised voltage-dependent source"); }
            }
        }
        //println!("*INFO* Non-linear stamped matrix");
        //self.pp_matrix(&m);
    }


    // stamp independent sources
    fn independent_source_stamp(&self, m: &mut [Vec<f64>], t_now: f64) {

        if !&self.independent_sources.is_empty() {
            trace!("  [STAMP] Stamping independent source elements");
        }

        for el in &self.independent_sources {
            match *el {
                circuit::Element::Isin(ref isrc) => {
                    trace!(" [STAMP] {}", el);

                    // evaluate at the present sim time
                    let i_now = isrc.evaluate(t_now);

                    // stamp
                    self.stamp_current_source(m, &circuit::CurrentSource{
                        p: isrc.p,
                        n: isrc.n,
                        value: i_now,
                    });
                },

                circuit::Element::Vsin(ref vsrc) => {
                    trace!("  [STAMP] {}", el);

                    // evaluate at the present sim time
                    let v_now = vsrc.evaluate(t_now);

                    // stamp
                    self.stamp_voltage_source(m, &circuit::VoltageSource{
                        p: vsrc.p,
                        n: vsrc.n,
                        value: v_now,
                        idx: vsrc.idx,
                    });
                },

                circuit::Element::Vpwl(ref vsrc) => {
                    trace!("  [STAMP] {}", el);

                    // evaluate at the present sim time
                    let v_now = vsrc.evaluate(t_now);

                    // stamp
                    self.stamp_voltage_source(m, &circuit::VoltageSource{
                        p: vsrc.p,
                        n: vsrc.n,
                        value: v_now,
                        idx: vsrc.idx,
                    });
                },

                _ => { println!("*ERROR* - unrecognised independent source element"); }
            }
        }
        //println!("*INFO* Independent source stamped matrix");
        //self.pp_matrix(&m);
    }


    // check for convergence by testing new and previous solutions against
    // RELTOL and the like
    pub fn convergence_check(
        &self,
        xv: &[f64],
        yv: &[f64],
        cfg: &analysis::Configuration,
    ) -> ConvergenceResult {

        let mut res = Ok(true);
        for (i,x) in xv.iter().enumerate() {
            if !x.is_finite() {
                println!("*ERROR* math gone bad - infinites in convergence check {}", i);
                res = Err(ConvergenceError::Divergent);
                continue;
            }
            let limit: f64 = if i < self.c_nodes {
                x.abs() * cfg.RELTOL + cfg.VNTOL
            } else {
                x.abs() * cfg.RELTOL + cfg.ABSTOL
            };

            let this = (x - yv[i]).abs();
            trace!("  [CONVERGE] {} < {} = {}", this, limit, (this < limit));
            if this > limit {
                res = Ok(false);
            }
        }
        trace!("  [CONVERGE] Convergence check: {:?}", res); 
        res
    }

}

