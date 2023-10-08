//! Datastructures for describing a Circuit
use std::fmt;
use std::collections::HashMap;

pub use crate::parameter::Parameter;

pub use crate::element::Element;
pub use crate::element::diode::Diode;
pub use crate::element::isine::CurrentSourceSine;
pub use crate::element::vsine::VoltageSourceSine;
pub use crate::element::capacitor::Capacitor;
pub use crate::element::resistor::Resistor;
pub use crate::element::independent::CurrentSource;
pub use crate::element::independent::VoltageSource;
pub use crate::element::vpwl::VoltageSourcePwl;


/// Program execution trace macro - prefix `<circuit>`
macro_rules! trace {
    ($fmt:expr $(, $($arg:tt)*)?) => {
        // uncomment the line below for tracing prints
        //println!(concat!("<circuit> ", $fmt), $($($arg)*)?);
    };
}


/// Physical Constant: Boltzman
pub const BOLTZMANN : f64 = 1.380_648_8e-23;

/// Physical Constant: Charge of an Electron
pub const CHARGE : f64 = 1.603e-19;

/// Simulator Constant: Minimum Impedance between Nodes
pub const GMIN : f64 = 1.0e-12;

/// Index of a node in the matrix
pub type NodeId = usize;


/// Subcircuit Instantiation
#[derive(Clone)]
pub struct Instance {
    pub name: String,
    pub subckt: String,
    pub conns: Vec<NodeId>,
    pub params: Vec<Parameter>,
}

impl Instance {

    pub fn new(name: &str, subckt: &str) -> Self {
        Instance {
            name: String::from(name),
            subckt: String::from(subckt),
            conns: vec![],
            params: vec![],
        }
    }

    pub fn add_connection(&mut self, nid: NodeId) {
        self.conns.push(nid);
    }

    pub fn add_parameter(&mut self, param: &Parameter) {
        self.params.push(param.clone());
    }

}

impl fmt::Display for Instance {
    fn fmt (&self, f:&mut fmt::Formatter) -> fmt::Result {
        // I can't do a node-name lookup here yet as the LUT is only
        // build at the end of a read...
        write!(f, "Inst '{}' of subcircuit '{}' has:\n connections {:?}\n parameters {:?}",
            self.name, self.subckt,
            self.conns, self.params)
    }
}

#[derive(Default, Clone)]
/// A Collection of Circuit Elements describing a circuit
pub struct Circuit {
    pub name: String,
    pub elements: Vec<Element>,
    pub nid_next: usize,
    pub v_idx_next: usize,
    pub nodes: HashMap<String, NodeId>,
    pub node_id_lut: HashMap<NodeId, String>,
    pub instances: Vec<Instance>,
    pub num_ports: usize,
    pub params: Vec<Parameter>,
}

impl Circuit {

    /// Initialise a new circuit description
    pub fn new() -> Circuit {
        let mut nodes = HashMap::new();
        nodes.insert(String::from("gnd"), 0);

        Circuit {
            name: String::from("<toplevel>"),
            elements: vec![],
            nid_next: 1, // skip 0 cos its gnd
            v_idx_next: 0,
            nodes,
            node_id_lut: HashMap::new(),
            instances: vec![],
            num_ports: 0,
            params: vec![],
        }
    }

    /// List the parameters of the circuit
    pub fn list_parameters(&self) {
        for param in &self.params {
            println!(" param: {} = {:?} ", param.name, param.expr);
        }
    }

    /// List the elements of the circuit
    pub fn list_elements(&self) {
        for el in &self.elements {
            println!(" elem: {}", el);
        }
    }

    /// List the nodes and associated node indices
    pub fn list_nodes(&self) {
        for (name, id) in &self.nodes {
            println!(" node: {} ({})", name, id);
        }
    }

    /// List the subcircuit instantiations
    pub fn list_instantiations(&self) {
        for inst in &self.instances {
            println!(" inst: {}", inst);
        }
    }

    /// Count the nodes in the circuit
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
                    Element::Vsin(VoltageSourceSine{ ref p, ref n, .. }) => {
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
                    Element::Vpwl(VoltageSourcePwl{ ref p, ref n, ..}) => {
                        if !seen[*p] {
                            seen[*p] = true;
                            c_nodes += 1;
                        }
                        if !seen[*n] {
                            seen[*n] = true;
                            c_nodes += 1;
                        }
                    }
                }
        }
        c_nodes
    }

    /// Count the voltage sources in the circuit
    ///
    /// Counts both `V` and `VSIN`.
    pub fn count_voltage_sources(&self) -> usize {

        // number of voltage sources in the circuit
        let mut c_vsrc: usize = 0;

        for el in &self.elements {
            match *el {
                Element::V(VoltageSource{..}) => {
                        c_vsrc += 1;
                },
                Element::Vsin(VoltageSourceSine{..}) => {
                        c_vsrc += 1;
                },
                Element::Vpwl(VoltageSourcePwl{..}) => {
                        c_vsrc += 1;
                },
                _ => {}
            }
        }
        c_vsrc
    }

    /// Add DC current source
    pub fn add_i(&mut self, p: NodeId, n: NodeId, value: f64) {
        self.elements.push(
            Element::I(CurrentSource{p, n, value})
        );
    }

    /// Add AC current source
    pub fn add_i_sin(&mut self, i_sin: CurrentSourceSine) {
        self.elements.push(Element::Isin(i_sin));
    }

    /// Add AC voltage source
    pub fn add_v_sin(&mut self, v_sin: VoltageSourceSine) {
        let mut v_sin_upd = v_sin.clone();
        v_sin_upd.idx = self.v_idx_next;
        self.elements.push(Element::Vsin(v_sin_upd));
        self.v_idx_next += 1;
    }

    /// Add piecewise linear voltage source
    pub fn add_v_pwl(&mut self, v_pwl: VoltageSourcePwl) {
        let mut v_pwl_upd = v_pwl.clone();
        v_pwl_upd.idx = self.v_idx_next;
        self.elements.push(Element::Vpwl(v_pwl_upd));
        self.v_idx_next += 1;
    }


    /// Add DC voltage source
    pub fn add_v(&mut self, p: NodeId, n: NodeId, value: f64) {
        self.elements.push(
            Element::V(VoltageSource{p, n, value, idx:self.v_idx_next})
        );
        self.v_idx_next += 1;
    }

    /// Add resistor
    pub fn add_r(&mut self, ident: String, a: NodeId, b: NodeId, value: f64) {
        self.elements.push(
            Element::R(Resistor{ident, a, b, value})
        );
    }

    /// Add capacitor
    pub fn add_c(&mut self, ident: String, a: NodeId, b: NodeId, value: f64) {
        self.elements.push(
            Element::C(Capacitor{ident, a, b, value})
        );
    }

    /// Add diode
    pub fn add_d(&mut self, d:Diode) {
        self.elements.push(Element::D(d));
    }

    /// Add an instantiation
    pub fn add_instance(&mut self, inst:Instance) {
        self.instances.push(inst);
    }

    /// Add a node
    /// Returns a node id. Only really adds the node if it isn't already
    /// on the list
    pub fn add_node(&mut self, name: &str) -> NodeId {
        if let Some(node_id) = self.get_node_id(name) {
            node_id
        } else {
            self.nodes.insert(String::from(name), self.nid_next);
            trace!("Add node '{}' with id {}", name, self.nid_next);
            self.nid_next += 1;
            self.nid_next - 1
        }
    }


    /// add a node alias
    pub fn add_node_alias(&mut self, name: &str, nid: NodeId) {

        trace!("Add node alias '{}' with id {}", name, nid);
        if let Some(node_id) = self.get_node_id(name) {
            if node_id != nid {
                println!("*ERROR* Can't add a node alias for a nonexisting NodeId");
                panic!();
            }
        }
        self.nodes.insert(String::from(name), nid);
    }


    /// Look up the `NodeId` for a node name
    pub fn get_node_id(&self, name: &str) -> Option<NodeId> {
    
        // Node name might be hierarchical. We look for an alias
        // of ground in the last bit.
        let endbit = name.rsplit_once('.');
        let localname = if let Some(bit) = endbit {
            bit.1
        } else {
            name
        };
        match localname {
            "gnd" | "GND" | "0" => Some(0),
            _ => self.nodes.get(name).copied()
        }
    }

    /// Build NodeId lookup
    /// Do this after reading in the SPICE circuit
    pub fn build_node_id_lut(&mut self) {
        for (name, id) in &self.nodes {
            self.node_id_lut.insert(*id, String::from(name));
        }
    }

    /// Find a parameter value
    ///
    /// `N` is small..
    pub fn get_param_value(&self, name: &str) -> Option<f64> {
        for p in &self.params {
            if p.name == name {
                return p.value
            }
        }
        None
    }

}
