//! Expand all the Subcircuits
//!
//! This crate has a bunch of routines to expand all the subcircuits from
//! the toplevel down. The toplevel circuit is what is described in the
//! SPICE deck, with the subcircuits being all the `Xsubckt1 ...`
//! instantiations.
//!
//! Work recursively from a clone of the toplevel schematic datastructure -
//! the "toplevel clone". At each level of hierarchy, add the circuit elements
//! from the subciruit to the toplevel clone.
//!
//! Instance names and netnames created in subciruits are renamed so the
//! identifiers include the hierarchy. For example, if a node called `out`
//! is needed for a circuit at hierarchy `X1.X1.X3` then a new node will
//! be created called `X1.X2.X3.out` and assigned a new `NodeId`.
//!
//! For ports, node aliases are added to the node list of the toplevel
//! clone. The new node name has the hierarchical prefix. The `NodeId`
//! is looked up from the connections in the instantiation.
//!
//! For example, consider the instantiation and subcircuit definition below:
//!
//! ```spice
//! X1 node1 node2 my_subckt
//!
//! .subckt my_subckt port1 port2
//!    ...blah...
//! .ends
//! ```
//!
//! The subcircuit's `port1` is connected to node `node1` in the above
//! subcircuit. We look up the `NodeId` for `node1` and find that it is
//! 69. We add this node alias to the node list of the toplevel clone:
//! `X1.port1 = 69`.
//!
//! If we find the subcircuit has subcircuits of its own, we push to the
//! hierarchy name stack and expand that subcircuit into the toplevel
//! clone.

use crate::element::{Element};
use crate::circuit::{Circuit, Instance};
use crate::circuit::{NodeId};

use crate::element::resistor::Resistor;
use crate::element::capacitor::Capacitor;
use crate::element::diode::Diode;

use crate::element::vdepsrc::{Vcvs, Vccs};

use crate::parameter::Parameter;
use crate::bracket_expression::{Expression};

/// Program execution trace macro - prefix `<expand>`
macro_rules! trace {
    ($fmt:expr $(, $($arg:tt)*)?) => {
        // uncomment the line below for tracing prints
        //println!(concat!("<expand> ", $fmt), $($($arg)*)?);
    };
}


/// Expand subcircuits.
///
/// This takes a list of subcircuits and returns a single `Circuit`
/// datastructure. The first circuit in the list is taken as the toplevel,
/// and all its subcircuits and all their subciruits are expanded into a
/// single circuit.
///
/// Since we don't know the hierarchy of the circuit up-front, this
/// function kicks off a cascade of recursive calls to `expand_subckt()`.
pub fn expand(ckts: &[Circuit]) -> Circuit {

    let mut ckt = ckts[0].clone();
    let hier: Vec<String> = vec![];

    trace!("------------------------------------------------");
    expand_instances(ckts, &mut ckt, 0, &hier);
    trace!("------------------------------------------------");

    ckt.build_node_id_lut();
    ckt

}


/// Expand the instances in the current scope.
///
/// Foreach instance in at this level
/// 1. Resolve parameter values
/// 2. either:
///   2a. `expand_primitive()` or
///   2b. `expand_subckt()`
///
/// Instances include both primitive circuit element and subcircuits
fn expand_instances(
    ckts: &[Circuit],
    ckt: &mut Circuit,
    ckt_id: usize,
    inhier: &[String], // the hierarchy we are working in
) {

    let mut hier = inhier.to_owned();

//    println!("-- Deal with instances of maybe subcircuits -- {} --",
//             inhier.len());
    trace!("expand_instances() -> {}", hier.join("."));

    // Set up aliases for parameters
    // When at scope X1.X2 which has instances Xa and Xbb, set param
    // aliases in `ckt` for:
    //
    //   X1.X2.xa.param0
    //             ...
    //   X1.X2.xa.paramN
    //
    //   X1.X2.xbb.param0
    //              ...
    //   X1.X2.xbb.paramM

    for inst in &ckts[ckt_id].instances {

        trace!("> inst: '{}' . '{}'", hier.join("."), inst.name);

        // resolve the parameters at this level
        // add the parameter to the main circuits parameter list in the same style
        // as for nodes: prefixed with the hierarchy...
        // this way, when we come to actually realize the subcircuit/primitive,
        // we can look up an f64 value for the parameter directly.

        // TODO: any parameters we can't set at this level, the lower level
        // will pick up a default?

        for p in &inst.params {

            hier.push(inst.name.to_string()); // inst-name
            hier.push(p.name.to_string()); // param-name
            let param_full_name = hier.join(".");
            hier.pop(); // param-name
            hier.pop(); // inst-name

            trace!("Resolving param {}", param_full_name);

            let param_override = match &p.expr {

                // look for a parameter override that has an expression:
                // `cval0={cval}`
                Some(Expression::Literal(val)) => {
                    Some(*val)
                },

                Some(Expression::Identifier(ident)) => {
                    // there is an identifier that must be looked up
                    // in the current scope

                    hier.push(ident.to_string()); // lut-name
                    let lookup_param_name = hier.join(".");
                    hier.pop(); // lut-name

                    let lut = ckt.get_param_value(&lookup_param_name);

                    #[allow(clippy::manual_map)] // cos of the trace!
                    if let Some(val) = lut {
                        trace!("Found value for identifier in param");
                        Some(val)
                    } else {
                        trace!("Can't find identifier '{}' for param '{}'",
                               lookup_param_name, param_full_name);
                        None
                    }
                },

                _ => {
                    None
                }
            };

            if let Some(value) = param_override {
                trace!("Subckt Parameter Override : {} = {}",
                       param_full_name, value);
                ckt.params.push( Parameter {
                    name: param_full_name,
                    defval: None,
                    expr: None,
                    value: Some(value),
                });
            };

        } // for p in parameters

        // Make sure we don't leave out any parameters that have defaults
        // and are not overridden. The problem is that I don't know if I'm a
        // subcircuit here...
        if inst.subckt != "/device" {
            trace!("Looking for nonoverridden parameters");

            // find the subckt definition index
            let subckt_id = if let Some(ckt_id) = find_subckt_index(ckts, &inst.subckt) {
                trace!("Found subcircuit definition: index={}; subckt={}; ident={}",
                   ckt_id, ckts[ckt_id].name, inst.name);
                ckt_id
            } else {
                println!("*FATAL* Can't find a definition for subcircuit {}",
                    inst.subckt);
                panic!();
            };

            for param_def in &ckts[subckt_id].params {

                hier.push(inst.name.to_string());
                hier.push(param_def.name.to_string()); // param-name
                let param_full_name = hier.join(".");
                hier.pop(); // param-name
                hier.pop(); // inst-name

                trace!("Looking for subckt inst param : {}", param_full_name);

                let lut = ckt.get_param_value(&param_full_name);

                if lut.is_some() {
                    trace!("param '{}' already defined, skipping", param_full_name);
                    continue;
                }
                trace!("Need a default from {:?}", param_def);

                let param_default_value = match &param_def.defval {

                    // look for a parameter override that has an expression:
                    // `cval0={cval}`
                    Some(Expression::Literal(val)) => {
                        Some(*val)
                    },

                    Some(Expression::Identifier(ident)) => {
                        // there is an identifier that must be looked up
                        // in the enclosing scope
                        hier.push(ident.to_string()); // lut-name
                        let lookup_param_name = hier.join(".");
                        hier.pop(); // lut-name

                        let lut = ckt.get_param_value(&lookup_param_name);
                        trace!("IIIIIIIIIIII {}", lookup_param_name);
                        if let Some(val) = lut {
                            trace!("HHHHHH asdfasdf-asdf HJASDFASDF");
                            Some(val)
                        } else {
                            panic!("*FATAL* Can't find prim override '{}'",
                                lookup_param_name);
                        }
                    },

                    _ => {
                        panic!("*FATAL* just can't get enough");
                    }
                };

                if let Some(value) = param_default_value {
                    trace!("Subckt Parameter Default : {} = {}",
                           param_full_name, value);
                    ckt.params.push( Parameter {
                        name: param_full_name,
                        defval: None,
                        expr: None,
                        value: Some(value),
                    });
                };
            } // for
        }


        // now we can expand primitives...
        if inst.subckt == "/device" {
            expand_primitive(ckts, ckt, ckt_id, inst, inhier);
        } else {
            expand_subckt(ckts, ckt, ckt_id, inst, &hier);
        }

    } // insts
}

/// Expand a subcircuit instantiation
/// Use this recursively
fn expand_subckt(
    ckts: &[Circuit],
    ckt: &mut Circuit,
    host_ckt_id: usize,
    inst: &Instance,
    inhier: &[String]
) {

    let mut hier = inhier.to_owned();

    trace!("expand_subckt() -> '{}' . '{}'", hier.join("."), inst.name);

    // find the subckt definition index
    let subckt_id = if let Some(ckt_id) = find_subckt_index(ckts, &inst.subckt) {
        trace!("Found subcircuit definition: index={}; subckt={}; ident={}",
           ckt_id, ckts[ckt_id].name, inst.name);
        ckt_id
    } else {
        println!("*FATAL* Can't find a definition for subcircuit {}",
            inst.subckt);
        panic!();
    };

    // check that the instantiation and the subckt agree on the
    // number of ports
    //dbg!(self.ckts[subckt_id].num_ports, inst.conns.len());
    if ckts[subckt_id].num_ports != inst.conns.len() {
        print!("*ERROR* Instantiation and subcircuit definitions");
        println!("have different port sizes");
    }


    // Add node aliases for all the ports
    for (n, hnid) in inst.conns.iter().enumerate() {
        trace!(" -> {} {}", n, hnid);
        let nid = n + 1;
        let port = &ckts[subckt_id].node_id_lut[&nid];

        hier.push(inst.name.to_string()); // inst-name
        hier.push(port.to_string()); // port-name
        let full_port_name = hier.join(".").to_string();
        hier.pop(); // port-name
        hier.pop(); // inst-name

        let host_net_name = &ckts[host_ckt_id].node_id_lut[hnid];

        hier.push(host_net_name.to_string()); // connecting-net
        let full_host_net_name = hier.join(".").to_string();
        hier.pop(); // connecting-net

        //println!("  '{}' -> '{}' ", full_port_name, full_host_net_name);
        let top_nid = ckt.add_node(&full_host_net_name);

        //println!(" '{}' -> {} -> '{}'", full_port_name, top_nid, full_host_net_name);
        ckt.add_node_alias(&full_port_name, top_nid);
    }

    hier.push(inst.name.to_string()); // inst-name
    expand_instances(ckts, ckt, subckt_id, &hier);
    hier.pop(); // inst-name
}


/// Expand a primitive instantiation
/// (not recursive)
//
// add all the elements from the subcircuit, but translate (or add)
// the node names they're connected to.
// * If the node is a port on the subcircuit, then the node
//   index already exists in the upper-level circuit.
// * If the node name is not a port, it needs to be a new
//   node name
//
// What are the port nets/ids in this subcircuit?
//
// We know that ports are pushed onto the nodelist first.
// 0 a b
// 0 a b int1 int2 int3
//
// a and b - look up the nodeid in the instantiation line
// int1,2,3 - add these as new nodes at the toplevel
//
// either way, update the NodeIds for the new element
fn expand_primitive(
    ckts: &[Circuit],
    ckt: &mut Circuit,
    host_ckt_id: usize,
    inst: &Instance,
    inhier: &[String] // the scope the primitive is instantiated in
) {

    let mut hier = inhier.to_owned();

    trace!("expand_primitive() -> {} . {}", hier.join("."), inst.name);

    // lookup connections in the scope in which the R, C whatever is instantiated
    let mut n: Vec<NodeId> = vec![];
    for net in &inst.conns {
        let tmp = local_connect(ckts, ckt, host_ckt_id, &hier, *net);
        n.push(tmp);
    }

    hier.push(inst.name.to_string()); // push the primitive local idenitifer
    let ident = hier.join("."); // full path to device

    if inst.name.starts_with('R') {
        trace!("Found a resistor primitive");
        assert!(!inst.params.is_empty());

        hier.push("/param0".to_string());
        let param_full_name = hier.join(".");
        hier.pop();

        let param_lut = ckt.get_param_value(&param_full_name);
        let value = if let Some(rval) = param_lut {
            rval
        } else {
            panic!("*FATAL* Value for R was not resolved");
        };

        let res = Resistor {ident, a: n[0], b: n[1], value};
        ckt.elements.push(Element::R(res));
    } else if inst.name.starts_with('C') {
        trace!("Found a capacitor primitive");
        assert!(!inst.params.is_empty());

        hier.push("/param0".to_string());
        let param_full_name = hier.join(".");
        hier.pop();

        let param_lut = ckt.get_param_value(&param_full_name);
        let value = if let Some(cval) = param_lut {
            cval
        } else {
            println!("Can't find {}", param_full_name);
            panic!("*FATAL* Value for C was not resolved");
        };

        let cap = Capacitor {ident, a: n[0], b: n[1], value };
        ckt.elements.push(Element::C(cap));
    } else if inst.name.starts_with('D') {
        trace!("Found a diode primitive");
        let i_sat = 1e-9;
        let tdegc = 27.0;
        let diode = Diode::new(&ident, n[0], n[1], i_sat, tdegc);
        ckt.elements.push(Element::D(diode));
    } else if inst.name.starts_with('E') {
        trace!("Found a vcvs primitive");
        assert!(!inst.params.is_empty());

        hier.push("/param0".to_string());
        let param_full_name = hier.join(".");
        hier.pop();

        let param_lut = ckt.get_param_value(&param_full_name);
        let k = if let Some(cval) = param_lut {
            cval
        } else {
            println!("Can't find {}", param_full_name);
            panic!("*FATAL* k value for VCVS was not resolved");
        };

        let vcvs = Vcvs {ident, p: n[0], n: n[1], cp: n[2], cn: n[3], k };
        ckt.elements.push(Element::Vcvs(vcvs));
    } else if inst.name.starts_with('G') {
        trace!("Found a vccs primitive");
        assert!(!inst.params.is_empty());

        hier.push("/param0".to_string());
        let param_full_name = hier.join(".");
        hier.pop();

        let param_lut = ckt.get_param_value(&param_full_name);
        let k = if let Some(cval) = param_lut {
            cval
        } else {
            println!("Can't find {}", param_full_name);
            panic!("*FATAL* k value for VCCS was not resolved");
        };

        let vccs = Vccs {ident, p: n[0], n: n[1], cp: n[2], cn: n[3], k };
        ckt.elements.push(Element::Vccs(vccs));
    } else {
        println!("*ERROR* Unrecognised primitive '{}'", inst.name);
        panic!("*FATAL*");
    }

    // should pop the primitive local instance name
    // this kinda doesn't matter cos we're not in a loop
    hier.pop(); // inst-name
}



/// Find the index of the subcircuit called `name`.
///
/// `N` is small, so just search the circuit list one by one until
/// a name match is found.
fn find_subckt_index(ckts: &[Circuit], name: &str) -> Option<usize> {
    for (i, ckt) in ckts.iter().enumerate() {
        if ckt.name == name {
            return Some(i);
        }
    }
    None
}


/// Connect a primitive
fn local_connect(
    ckts: &[Circuit],
    ckt: &mut Circuit,
    host_ckt_id: usize,
    inhier: &[String],
    lnid: NodeId,
) -> NodeId {

    let mut hier = inhier.to_owned();

    let local_node_name = &ckts[host_ckt_id].node_id_lut[&lnid];
    hier.push(local_node_name.to_string());
    let node_name = hier.join(".");
    _ = hier.pop();

    #[allow(clippy::manual_map)] // cos of the trace!
    let gnid = ckt.add_node(&node_name);
    trace!("Primitive connection: '{}' -> {}", node_name, gnid);
    gnid

}
