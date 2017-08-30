
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

        println!("*INFO* Building Voltage Node Matrix and Current Vector");

        // Voltage Node Matrix
        // I think I have to make this out of Vecs (on the heap) because c_nodes is
        // not known at compile time. Makes sense, I suppose - could blow the stack if
        // c_nodes is any way huge.
        // [ V I ]
        let mut v = vec![ vec![0.0; c_nodes+1]; c_nodes]; // +1 for currents
        let i = c_nodes; // index for current vector

        // Fill up the voltage node and current vector
        // This needs to know about each of the kinds of circuit elements, so
        // the node equations can be built up appropriately.
        for el in &ckt.elements {
            match *el {
                circuit::Element::I(circuit::CurrentSource{ ref p, ref n, ref value }) => {
                    println!("  [ELEMENT] Current source: {} into node {} and out of node {}",
                            value, p, n);
                    v[*p][i] = v[*p][i] + value; // += doesn't work here
                    v[*n][i] = v[*n][i] - value;
                }
                circuit::Element::R(circuit::Resistor{ ref a, ref b, ref value }) => {
                    println!("  [ELEMENT] Resistor");
                    let over = 1.0 / value;
                    v[*a][*a] = v[*a][*a] + over;
                    v[*a][*b] = v[*a][*b] - over;
                    v[*b][*a] = v[*b][*a] + over;
                    v[*b][*b] = v[*b][*b] - over;
                }
                
            }
        }
        println!("{:?}", v);

        // naive implementation of gaussian elimination
        println!("*INFO* Gaussian Elimination");
       
        // back-substitution
        println!("*INFO* Back-substitution");

    }

}

