
use circuit;


pub fn banner() {

    println!("********************************************");
    println!("***       Tiny-SPICE-Simulator           ***");
    println!("***        (c) CrapCorp 2017             ***");
    println!("*** Patent Pending, All rights reserved  ***");
    println!("********************************************");

}

pub struct Engine {
    next_id: circuit::NodeId,
}

impl Engine {

    pub fn new() -> Engine {
        Engine {
            next_id: 1,
        }
    }

    pub fn elaborate(&mut self, ckt: &circuit::Circuit) {
        // assume here that nodes have been indexed 0 -> N-1
        // where n is the number of nodes (including ground) in the circuit

        // Number of nodes, including ground (aka 0, aka gnd)
        let mut c_nodes = ckt.count_nodes();
        println!("*INFO* There are {} nodes in the design, including ground", c_nodes);

        println!("*INFO* Building Voltage Node Matrix and Current Vector");

        // Voltage Node Matrix
        type AdjacencyList = Vec<circuit::NodeId>;
        let mut adj_matrix: Vec<AdjacencyList> = vec![ vec![] ];

/*

        for el in &ckt.elements {
            match *el {
                circuit::Element::I(circuit::CurrentSource{ ref p, ref n, ref value }) => {

                    println!("ADSF");
                }
                _ => {
                    println!("Ignoring");
                }
            }
        }
*/


    }

}

