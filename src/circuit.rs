use std::fmt;

pub use diode::Diode;
pub use isine::CurrentSourceSine;
pub use capacitor::Capacitor;

pub type NodeId = usize;

pub const BOLTZMANN : f64 = 1.3806488e-23;
pub const CHARGE : f64 = 1.603e-19;
pub const GMIN : f64 = 1.0e-12;

#[allow(dead_code)]
pub struct Resistor {
    pub a: NodeId,
    pub b: NodeId,
    pub value: f64, // Ohms
}


#[allow(dead_code)]
pub struct CurrentSource {
    pub p: NodeId,
    pub n: NodeId,
    pub value: f64, // Amperes
}

#[allow(dead_code)]
pub struct VoltageSource {
    pub p: NodeId,
    pub n: NodeId,
    pub value: f64, // Volts
}

#[allow(dead_code)]
pub enum Element {
    R(Resistor),
    I(CurrentSource),
    V(VoltageSource),
    D(Diode),
    Isin(CurrentSourceSine),
    C(Capacitor),
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
            Element::V(ref el) => {
                write!(f, "V a:{} b:{} {}Volts", el.p, el.n, el.value)
            },
            Element::D(ref el) => {
                write!(f, "D p:{} n:{} I_sat={}A", el.p, el.n, el.i_sat)
            },
            Element::Isin(ref el) => {
                write!(f, "Isin p:{} n:{} = {} + {} * sin(2pi {})",
                    el.p, el.n, el.vo, el.va, el.freq)
            },
            Element::C(ref el) => {
                write!(f, "C a:{} b:{} {}Farads", el.a, el.b, el.value)
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
                    Element::V(VoltageSource{ ref p, ref n, .. }) => {
                        if !seen[*p] {
                            seen[*p] = true;
                            c_nodes += 1;
                        }
                        if !seen[*n] {
                            seen[*n] = true;
                            c_nodes += 1;
                        }
                    }
                    Element::D(Diode{ ref p, ref n, .. }) => {
                        if !seen[*p] {
                            seen[*p] = true;
                            c_nodes += 1;
                        }
                        if !seen[*n] {
                            seen[*n] = true;
                            c_nodes += 1;
                        }
                    }
                    Element::Isin(CurrentSourceSine{ ref p, ref n, .. }) => {
                        if !seen[*p] {
                            seen[*p] = true;
                            c_nodes += 1;
                        }
                        if !seen[*n] {
                            seen[*n] = true;
                            c_nodes += 1;
                        }
                    }
                    Element::C(Capacitor{ ref a, ref b, .. }) => {
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

    pub fn count_voltage_sources(&self) -> usize {

        // number of voltage sources in the circuit
        let mut c_vsrc: usize = 0;

        for el in &self.elements {
                match *el {
                    Element::V(VoltageSource{..}) => {
                        c_vsrc += 1;
                        }
                    _ => {
                    }
                }
        }
        c_vsrc
    } 

    /// Add DC current source
    pub fn add_i(&mut self, p: NodeId, n: NodeId, value: f64) {
        self.elements.push(
            Element::I(CurrentSource{p: p, n: n, value: value})
        );
    }

    /// Add resistor
    pub fn add_r(&mut self, a: NodeId, b: NodeId, value: f64) {
        self.elements.push(
            Element::R(Resistor{a: a, b: b, value: value})
        );
    }


}
