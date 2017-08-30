use std::fmt;

pub type NodeId = usize;


#[allow(dead_code)]
pub struct Resistor {
    pub a: NodeId,
    pub b: NodeId,
    pub value: f32, // Ohms
}


#[allow(dead_code)]
pub struct CurrentSource {
    pub p: NodeId,
    pub n: NodeId,
    pub value: f32, // Amperes
}


#[allow(dead_code)]
pub enum Element {
    R(Resistor),
    I(CurrentSource),
}


impl fmt::Display for Element {
    fn fmt (&self, f:&mut fmt::Formatter) -> fmt::Result {
        match *self {
            Element::I(ref el) => {
                write!(f, "I p:{} n:{} {}A", el.p, el.n, el.value)
            },
            Element::R(ref el) => {
                write!(f, "R a:{} b:{} {}Ohms", el.a, el.b, el.value)
            },
        }
    }
}


pub struct Circuit {
    pub elements: Vec<Element>,
}

impl Circuit {

    pub fn new() -> Circuit {
        Circuit {
            elements: vec![],
        }
    }

    pub fn show(&self) {
        for el in &self.elements {
            println!("{}", el);
        }
    }

    pub fn count_nodes(&self) -> usize {

        // number of nodes in the circuit - there is always at least ground
        let mut c_nodes: usize = 1;

        let mut seen = [false; 256]; // max nodes magic number
        seen[0] = true; // always a ground

        for el in &self.elements {
                match *el {
                    Element::I(CurrentSource{ ref p, ref n, .. }) => {
                        if !seen[*p] {
                            seen[*p] = true;
                            c_nodes += 1;
                        }
                        if !seen[*n] {
                            seen[*n] = true;
                            c_nodes += 1;
                        }
                    }
                    Element::R(Resistor{ ref a, ref b, .. }) => {
                        if !seen[*a] {
                            seen[*a] = true;
                            c_nodes += 1;
                        }
                        if !seen[*b] {
                            seen[*b] = true;
                            c_nodes += 1;
                        }
                    }
                }
        }
        c_nodes
    } 




}
