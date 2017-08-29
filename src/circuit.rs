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

}
