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

use crate::parameter::Parameter;
use crate::bracket_expression::{Expression};

/// Program execution trace macro - prefix `<expand>`
macro_rules! trace {
    ($fmt:expr $(, $($arg:tt)*)?) => {
        // uncomment the line below for tracing prints
        println!(concat!("<expand> ", $fmt), $($($arg)*)?);
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

    println!("------------------------------------------------");
    expand_instances(ckts, &mut ckt, 0, &hier);
    println!("------------------------------------------------");

    ckt.build_node_id_lut();
    ckt

}


fn expand_instances(
    ckts: &[Circuit],
    ckt: &mut Circuit,
    ckt_id: usize,
    inhier: &[String]
) {
    println!("-- Deal with instances of maybe subcircuits ---------");
    let insts: &Vec<Instance> = &ckts[ckt_id].instances.clone();

    let mut hier = inhier.to_owned();

    for inst in insts {

        // resolve the parameters at this level
        // add the parameter to the main circuits parameter list in the same style
        // as for nodes: prefixed with the hierarchy...
        hier.push(inst.name.to_string());
        for p in &inst.params {
            hier.push(p.name.to_string());

            let value = match p.expr {

                Some(Expression::Literal(val)) => {
                    val
                },

                _ => {
                    panic!("I'm getting to old for this shit");
                }
            };

            let param_full_name = hier.join(".");
            trace!("Considering: {} = {}", param_full_name, value);
            ckt.params.push( Parameter {
                name: param_full_name,
                defval: None,
                expr: None,
                value: Some(value),
            });

            hier.pop();
        }

        // now we can expand primitives...
        if inst.subckt == "/device" {
            expand_primitive(ckts, ckt, ckt_id, inst, inhier);
        } else {
            // ... and subcircuits
            let mut hier = inhier.to_owned();
            hier.push(inst.name.to_string());
            expand_subckt(ckts, ckt, ckt_id, inst, &hier);
        }

        hier.pop();
    }
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

    trace!("Expanding: {}", inst);
    let mut subckt_id = 0;

    // find the subckt definition index
    if let Some(ckt_id) = find_subckt_index(ckts, &inst.subckt) {
        trace!("Found definition for {}", inst.subckt);
        subckt_id = ckt_id;
    } else {
        println!("*ERROR* Can't find a definition for subcircuit {}",
            inst.subckt);
    }

    // check that the instantiation and the subckt agree on the
    // number of ports
    //dbg!(self.ckts[subckt_id].num_ports, inst.conns.len());
    if ckts[subckt_id].num_ports != inst.conns.len() {
        print!("*ERROR* Instantiation and subcircuit definitions");
        println!("have different port sizes");
    }

    trace!("Subcircuit: index={}; subckt={}; idenr={}",
           subckt_id, ckts[subckt_id].name, inst.name);

    // Add node aliases for all the ports
    for (n, hnid) in inst.conns.iter().enumerate() {
        trace!(" -> {} {}", n, hnid);
        let nid = n + 1;
        let port = &ckts[subckt_id].node_id_lut[&nid];

        hier.push(port.to_string());
        let full_port_name = hier.join(".").to_string();
        hier.pop();

        let host_net_name = &ckts[host_ckt_id].node_id_lut[hnid];
        hier.pop();
        hier.push(host_net_name.to_string());

        let full_host_net_name = hier.join(".").to_string();
        hier.pop();
        hier.push(inst.name.to_string());

        //println!("  '{}' -> '{}' ", full_port_name, full_host_net_name);
        let top_nid = ckt.add_node(&full_host_net_name);

        //println!(" '{}' -> {} -> '{}'", full_port_name, top_nid, full_host_net_name);
        ckt.add_node_alias(&full_port_name, top_nid);
    }

    // Resolve Parameters, somehow
    trace!("Resolving parameters...");
    println!(" Host  : {:?}", ckts[host_ckt_id].params);
    println!(" Subckt: {:?}", ckts[subckt_id].params);

    expand_instances(ckts, ckt, subckt_id, &hier);
    _ = hier.pop();
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
    inhier: &[String]
) {

    let mut hier = inhier.to_owned();

    println!("{}", inst);

    hier.push(inst.name.to_string()); // push the primitive local idenitifer
    let ident = hier.join(".");

    hier.pop();
    let n1 = local_connect(ckts, ckt, host_ckt_id, &hier, inst.conns[0]);
    let n2 = local_connect(ckts, ckt, host_ckt_id, &hier, inst.conns[1]);
    hier.push(inst.name.to_string());

    if inst.name.starts_with('R') {
        trace!("Found a resistor primitive");
        assert!(!inst.params.is_empty());

        hier.push("/param0".to_string());
        let param_full_name = hier.join(".");
        hier.pop();

        let param_lut = get_param_value(ckt, &param_full_name);
        let value = if let Some(rval) = param_lut {
            rval
        } else {
            println!("Can't find {}", param_full_name);
            panic!("*FATAL* Value for R was not resolved");
        };

        let res = Resistor {ident, a: n1, b: n2, value};
        ckt.elements.push(Element::R(res));
    } else if inst.name.starts_with('C') {
        trace!("Found a capacitor primitive");
        assert!(!inst.params.is_empty());

        hier.push("/param0".to_string());
        let param_full_name = hier.join(".");
        hier.pop();

        let param_lut = get_param_value(ckt, &param_full_name);
        let value = if let Some(cval) = param_lut {
            cval
        } else {
            println!("Can't find {}", param_full_name);
            panic!("*FATAL* Value for C was not resolved");
        };

        let cap = Capacitor {ident, a: n1, b: n2, value };
        ckt.elements.push(Element::C(cap));
    } else if inst.name.starts_with('D') {
        trace!("Found a diode primitive");
        let i_sat = 1e-9;
        let tdegc = 27.0;
        let diode = Diode::new(&ident, n1, n2, i_sat, tdegc);
        ckt.elements.push(Element::D(diode));
    } else {
        println!("*ERROR* Unrecognised primitive '{}'", inst.name);
        panic!("*FATAL*");
    }

    // should pop the primitive local instance name
    hier.pop();
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

/// Find a parameter value
///
/// `N` is small..
fn get_param_value(ckt: &Circuit, name: &str) -> Option<f64> {
    for p in &ckt.params {
        if p.name == name {
            return p.value
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

    let gnid = ckt.add_node(&node_name);
    trace!("Primitive connection: '{}' -> {}", node_name, gnid);
    gnid

}
