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
    expand_subckt(ckts, &mut ckt, 0, &hier);
    println!("------------------------------------------------");

    ckt.build_node_id_lut();
    ckt

}


/// Expand a subcircuit instantiation
/// Use this recursively
fn expand_subckt(
    ckts: &[Circuit],
    ckt: &mut Circuit,
    host_ckt_id: usize,
    inhier: &[String]
) {

    let mut hier = inhier.to_owned();

    println!("-- Deal with subcircuits -----------------------");
    let insts: &Vec<Instance> = &ckts[host_ckt_id].instances.clone();
    for inst in insts {
        println!("{}", inst);
        let mut subckt_id = 0;

        hier.push(inst.name.to_string());

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

        trace!("Subcircuit index: {}", subckt_id);

        // Add node aliases for all the ports
        for (n, hnid) in inst.conns.iter().enumerate() {
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
            let top_nid = ckt.nodes[&full_host_net_name];
            //println!(" '{}' -> {} -> '{}'", full_port_name, top_nid, full_host_net_name);
            ckt.add_node_alias(&full_port_name, top_nid);
        }

        // add all the elements from the subcircuit, but translate (or add)
        // the node names they're connected to.
        // * If the node is a port on the subcircuit, then the node
        //   index already exists in the upper-level circuit.
        // * If the node name is not a port, it needs to be a new
        //   node name

        // What are the port nets/ids in this subcircuit?

        // We know that ports are pushed onto the nodelist first.
        // 0 a b
        // 0 a b int1 int2 int3
        //
        // a and b - look up the nodeid in the instantiation line
        // int1,2,3 - add these as new nodes at the toplevel
        //
        // either way, update the NodeIds for the new element
        for subckt_el in &ckts[subckt_id].elements {
                trace!(" Element: {}", subckt_el);

                match subckt_el {

                    Element::R(subckt_res) => {
                        trace!("Found a resistor subcircuit element");

                        // Copy R, cos we have to tweak the nodeids for its ports
                        let mut res = subckt_res.clone();
                        hier.push(res.ident);
                        res.ident = hier.join(".");
                        hier.pop();

                        res.a = connect(ckts, ckt, inst, host_ckt_id,
                            subckt_id, &hier, subckt_res.a);
                        res.b = connect(ckts, ckt, inst, host_ckt_id,
                            subckt_id, &hier, subckt_res.b);
                        ckt.elements.push(Element::R(res));
                    },

                    Element::C(subckt_cap) => {
                        trace!("Found a capacitor subcircuit element");

                        // Copy element, cos we have to tweak the nodeids for its ports
                        let mut cap = subckt_cap.clone();
                        hier.push(cap.ident);
                        cap.ident = hier.join(".");
                        hier.pop();

                        cap.a = connect(ckts, ckt, inst, host_ckt_id,
                            subckt_id, &hier, subckt_cap.a);
                        cap.b = connect(ckts, ckt, inst, host_ckt_id,
                            subckt_id, &hier, subckt_cap.b);
                        ckt.elements.push(Element::C(cap));
                    },

                    Element::D(subckt_diode) => {
                        trace!("Found a diode subcircuit element");

                        // Copy element, cos we have to tweak the nodeids for its ports
                        let mut diode = subckt_diode.clone();
                        hier.push(diode.ident);
                        diode.ident = hier.join(".");
                        hier.pop();

                        diode.p = connect(ckts, ckt, inst, host_ckt_id,
                            subckt_id, &hier, subckt_diode.p);
                        diode.n = connect(ckts, ckt, inst, host_ckt_id,
                            subckt_id, &hier, subckt_diode.n);
                        ckt.elements.push(Element::D(diode));
                    },

                    _ => {},
                }

        }

        expand_subckt(ckts, ckt, subckt_id, &hier);
        _ = hier.pop();
    }
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


/// Connect a subcircuit element port
///
/// Find out if there's an existing node that should be connected to,
/// (port) or create a new node to connect to (internal node).
fn connect(
    ckts: &[Circuit],
    ckt: &mut Circuit,
    inst: &Instance,
    host_ckt_id: usize,
    subckt_id: usize,
    inhier: &[String],
    subckt_nid: NodeId,
) -> NodeId {

    let mut hier = inhier.to_owned();

    if subckt_nid > ckts[subckt_id].num_ports {
        let local_node_name = &ckts[subckt_id].node_id_lut[&subckt_nid];
        hier.push(local_node_name.to_string());
        let node_name = hier.join(".");
        _ = hier.pop();

        let nid = ckt.add_node(&node_name);
        trace!("Connected an internal subckt node: '{}' -> {}", node_name, nid);
        nid
    } else {
        // we have a port
        // create a node alias to the port
        // 1. find the netname in the host circuit
        let hnid = inst.conns[subckt_nid-1];
        hier.pop();
        let local_node_name = &ckts[host_ckt_id].node_id_lut[&hnid];
        hier.push(local_node_name.to_string());
        let node_name = hier.join(".");
        _ = hier.pop();
        hier.push(inst.name.to_string());

        let nid = ckt.add_node(&node_name); // should exist
        trace!("Connected a subckt port: '{}' -> {}", node_name, nid);
        nid
    }

}


