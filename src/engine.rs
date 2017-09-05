
use circuit;


pub fn banner() {

    println!("********************************************");
    println!("***       Tiny-SPICE-Simulator           ***");
    println!("***        (c) CrapCorp 2017             ***");
    println!("*** Patent Pending, All rights reserved  ***");
    println!("********************************************");

}

pub struct Engine {
    //next_id: circuit::NodeId,
}

impl Engine {

    pub fn new() -> Engine {
        Engine {
           //next_id: 1,
        }
    }

    pub fn elaborate(&mut self, ckt: &circuit::Circuit) {
        // assume here that nodes have been indexed 0 -> N-1
        // where n is the number of nodes (including ground) in the circuit

        // Number of nodes, including ground (aka 0, aka gnd)
        let c_nodes = ckt.count_nodes();
        println!("*INFO* There are {} nodes in the design, including ground", c_nodes);

        // Number of voltage sources in the design
        let c_vsrcs = ckt.count_voltage_sources();
        println!("*INFO* There are {} voltage sources in the design", c_vsrcs);

        println!("\n*INFO* Building Voltage Node Matrix and Current Vector");

        // Modified Nodal Analysis (MNA) Matrix
        let c_mna = c_nodes + c_vsrcs;
        // I think I have to make this out of Vecs (on the heap) because c_nodes is
        // not known at compile time. Makes sense, I suppose - could blow the stack if
        // c_nodes is any way huge.
        // [ V I ]
        let mut v = vec![ vec![0.0; c_mna+1]; c_mna]; // +1 for currents
        let ia = c_mna; // index for ampere vector

        // Fill up the voltage node and current vector
        // This needs to know about each of the kinds of circuit elements, so
        // the node equations can be built up appropriately.
        let mut i_vsrc : usize = c_nodes; // index, not amperage...
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
                        v[*p][ia] = v[*p][ia] - value; // -= doesn't work here
                    }
                    if *n != 0 {
                        v[*n][ia] = v[*n][ia] + value;
                    }
                }
                circuit::Element::R(circuit::Resistor{ ref a, ref b, ref value }) => {
                    println!("  [ELEMENT] Resistor");
                    let over = 1.0 / value;

                    // out of node 'a'
                    if *a != 0 {
                        v[*a][*a] = v[*a][*a] + over;
                        if *b != 0 {
                            v[*a][*b] = v[*a][*b] - over;
                        }
                    }

                    // out of node 'b'
                    if *b != 0 {
                        v[*b][*b] = v[*b][*b] + over;
                        if *a != 0 {
                            v[*b][*a] = v[*b][*a] - over;
                        }
                    }
                }
                circuit::Element::V(circuit::VoltageSource{ ref p, ref n, ref value }) => {
                    println!("  [ELEMENT] Voltage source: {}V from node {} to node {}",
                            value, p, n);

                    // put the voltage value in the 'known' vector
                    v[i_vsrc][ia] = *value;

                    let p_not_grounded = (*p != 0);
                    let n_not_grounded = (*n != 0);

                    if p_not_grounded {
                        v[i_vsrc][*p] = 1.0;
                        v[*p][i_vsrc] = 1.0;
                    }

                    if n_not_grounded {
                        v[i_vsrc][*n] = -1.0;
                        v[*n][i_vsrc] = -1.0;
                    }

                    i_vsrc += 1; // voltage source matrix index update 
                    
                }
                
            }
        }
        self.pp_matrix(&v);

        // naive implementation of gaussian elimination
        // From `Introduction to Algorithms`, page 818
        // "We start by subtracting multiples of the first equation from the other
        // equations in order to remove the first variable from those equations.
        // Then, we subtract multiples of the 2nd equation from the 3rd and 
        // subsequent equations so now the 1st and 2nd variables are removed from
        // them. 
        // Divide by zeros everywhere...
        println!("\n*INFO* Gaussian Elimination");
        for r_ref in 0..c_mna-1 { // row we're scaling
            if v[r_ref][r_ref] == 0.0 {
                //println!("Skipping v[{}][..]", r_ref);
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
        for i_res in 1..c_nodes {
            println!(" v[{:2}] = {}", i_res, n[i_res]);
        }
        for i_res in c_nodes..c_nodes+c_vsrcs {
            println!(" i[{:2}] = {}", i_res, n[i_res]);
        }

    }

    fn pp_matrix( &self, m : &Vec<Vec<f32>> ) {
        for r in m {
            for val in r {
                print!("{:.3}   ", val);
            }
            println!("");
        }
    }

}

