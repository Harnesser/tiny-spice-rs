//! Read a SPICE Deck
//!
//! Supported:
//! 1. Initial comment line
//! 2. Components:
//!   * Voltage source : `V<ident> <n+> <n-> <value>`
//!   * Current source : `I<ident> <n+> <n-> <value>`
//!   * Resistor : `R<ident> <n1> <n2> <value>`
//! 3. Node names:
//!   * Integers for now
//! 4. Values:
//!   * floating point with optional engineering scaler and unit
//! 5. Control Blocks:
//!   Only one operation for now - no sequences
//!   * DC Operating Point `op`
//!   * Transient : `trans <t_step> <t_stop> [t_start]`
//!     * for now, `t_start` is ignored
//! 6. Options (in Control Blocks)
//!   * Options: `option <OPTION_NAME> = <value>`
//!     * `ABSTOL`
//!     * `RELTOL`

use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};

use crate::circuit::{Circuit, Diode, CurrentSourceSine, VoltageSourceSine};
use crate::circuit::{Instance, Element};
use crate::analysis::{Configuration, Kind};

macro_rules! trace {
    ($fmt:expr $(, $($arg:tt)*)?) => {
        // uncomment the line below for tracing prints
        //println!(concat!("<spice> ", $fmt), $($($arg)*)?);
    };
}


/// Datastructure to info parsed from the SPICE deck
pub struct Reader {
    /// Circuit information
    /// `ckts[0]` is the toplevel
    ckts: Vec<Circuit>,
    /// Options and analysis commands
    cfg: Configuration,
    /// Irack which (sub)circuit we're adding things too.
    c: usize,
    /// Flag if problems were encountered during parsing
    there_are_errors: bool
}


impl Reader {

    #[allow(clippy::new_without_default)]
    pub fn new() -> Reader {
        let topckt = Circuit::new();
        Reader {
            ckts: vec![topckt],
            cfg: Configuration::new(),
            c: 0, // toplevel
            there_are_errors: false,
        }
    }

    /// Open and read a SPICE deck
    pub fn read(&mut self, filename :&Path) -> bool {

        let input = File::open(filename).unwrap();
        let buf = BufReader::new(input);
        let mut lines_iter = buf.lines();
        let mut in_control_block = false;
        let mut in_subckt = false;


        // circuit name is the SPICE name without any
        let ckt_name = filename
            .file_stem().expect("can't get stem of SPICE file path")
            .to_str().expect("cant stringify SPICE filename");
        self.cfg.ckt_name = ckt_name.to_string();
        self.ckts[0].name = ckt_name.to_string();
        println!("*INFO* Reading SPICE file: '{}'", filename.display());

        // first line is a comment and is ignored
        lines_iter.next();

        for line_wr in lines_iter {
            let line = line_wr.unwrap();
            let all_bits :Vec<&str> = line.split_whitespace().collect();

            // jump blank lines
            if all_bits.is_empty() {
                continue;
            }

            // skip comment lines
            if all_bits[0].starts_with('*') {
                continue;
            }

            // strip comments off the end
            let mut bits: Vec<&str> = vec![];
            for b in all_bits {
                if b == ";" {
                    break;
                }
                bits.push(b);
            }

            trace!("Bits: {:?}", bits);

            // let's go
            if in_control_block {
                trace!("*INFO* Parsing control '{}'", bits[0]);
                if bits[0] == "op" {
                    self.cfg.kind = Some(Kind::DcOperatingPoint);
                    let wavefile = Path::new("waves")
                        .join(&self.cfg.ckt_name)
                        .join("dc.dat");
                    self.cfg.set_wavefile(wavefile.to_str().expect("CasdfasDF"));

                } else if bits[0] == "tran" {
                    self.cfg.kind = Some(Kind::Transient);
                    let wavefile = Path::new("waves")
                        .join(&self.cfg.ckt_name)
                        .join("tran.dat");
                    self.cfg.set_wavefile(wavefile.to_str().expect("CasdfasDF"));

                    // step stop <start>
                    if bits.len() < 3 {
                        println!("*ERROR* not enough trans info");
                        self.there_are_errors = true;
                    }
                    self.cfg.TSTEP = extract_value(bits[1]).unwrap();
                    self.cfg.TSTOP = extract_value(bits[2]).unwrap();
                    if bits.len() > 3 {
                        self.cfg.TSTART = extract_value(bits[3]).unwrap();
                    }
                } else if bits[0] == "option" {
                        self.extract_option(&bits);
                } else if bits[0] == ".endc" {
                    in_control_block = false;
                } else {
                    println!("*WARN* Ignoring unrecognised command '{}'", bits[0]);
                }
            } else {

                // find out what we're looking at
                if bits[0] == ".ends" {
                    trace!("Leaving subcircuit");
                    if !in_subckt {
                        println!("*ERROR* .ends without a .subckt");
                        self.there_are_errors = true;
                    }
                    in_subckt = false;
                    self.c = 0; // point back to toplevel
                } else if bits[0].starts_with('I') {
                    let _ = extract_identifier(bits[0]);
                    let node1 = self.extract_node(bits[1]);
                    let node2 = self.extract_node(bits[2]);
                    if bits.len() == 4 {
                        trace!("*INFO* Idc");
                        let value = extract_value(bits[3]);
                        self.ckts[self.c].add_i(node1, node2, value.unwrap());
                    } else if bits[3].starts_with("SIN") {
                        trace!("*INFO* Isin");
                        let src = self.extract_i_sine(&bits);
                        self.ckts[self.c].add_i_sin(src);
                    }
                } else if bits[0].starts_with('V') {
                    let _ = extract_identifier(bits[0]);
                    let node1 = self.extract_node(bits[1]);
                    let node2 = self.extract_node(bits[2]);
                    if bits.len() == 4 {
                        trace!("*INFO* Vdc");
                        let value = extract_value(bits[3]);
                        self.ckts[self.c].add_v(node1, node2, value.unwrap());
                    } else if bits[3].starts_with("SIN(") {
                        trace!("*INFO* Vsin");
                        let src = self.extract_v_sine(&bits);
                        self.ckts[self.c].add_v_sin(src);
                    }
                } else if bits[0].starts_with('R') {
                    let ident = extract_identifier(bits[0]);
                    let node1 = self.extract_node(bits[1]);
                    let node2 = self.extract_node(bits[2]);
                    let value = extract_value(bits[3]);
                    self.ckts[self.c].add_r(ident, node1, node2, value.unwrap());
                } else if bits[0].starts_with('C') {
                    let ident = extract_identifier(bits[0]);
                    let node1 = self.extract_node(bits[1]);
                    let node2 = self.extract_node(bits[2]);
                    let value = extract_value(bits[3]);
                    self.ckts[self.c].add_c(ident, node1, node2, value.unwrap());
                } else if bits[0].starts_with('D') {
                    let d = self.extract_diode(&bits);
                    self.ckts[self.c].add_d(d);
                } else if bits[0].starts_with('X') {
                    trace!("Found instantiation");
                    let inst = self.extract_instance(&bits);
                    self.ckts[self.c].add_instance(inst);
                } else if bits[0].starts_with('.') {
                    if bits[0] == ".control" {
                        in_control_block = true;
                    } else if bits[0] == ".subckt" {
                        trace!("In subcircuit definition");

                        if self.c != 0 {
                            println!("*ERROR* Can't define a subckt in subckt");
                            self.there_are_errors = true;
                        }

                        // create a new circuit object for the subcircuit we're
                        // about to read in...
                        let subckt = Circuit::new();
                        self.ckts.push(subckt);
                        self.c = self.ckts.len() - 1;

                        self.ckts[self.c].name = bits[1].to_string();

                        // the port names are nodes in the subckt
                        let mut num_ports = 0;
                        for nn in bits.iter().skip(2) {
                            self.ckts[self.c].add_node(nn);
                            num_ports += 1;
                        }
                        self.ckts[self.c].num_ports = num_ports;

                        //self.ckts[0].add_subckt(subckt);
                        in_subckt = true;
                    } else {
                        println!("*ERROR* unsupported dot-command: {}", bits[0]);
                        self.there_are_errors = true;
                    }
                }
            }

            //for bit in bits {
            //    println!("->{}", bit);
            //}

            //println!("{}", line);
        }

        println!("Number of subcircuit definitions: {}", self.ckts.len()-1);
        for ckt in &self.ckts {
            println!("\nCircuit: {}", ckt.name);
            println!(" Ports: {}", ckt.num_ports);
            ckt.list_nodes();
            ckt.list_elements();
            ckt.list_instantiations();
        }
        println!();

        // build the `NodeId` -> name lookup tables for the toplevel circuit
        // and the all the subcircuits.
        for ckt in &mut self.ckts {
            ckt.build_node_id_lut();
        }

        self.there_are_errors
    }

    /// Parse a diode SPICE instantiation
    fn extract_diode(&mut self, bits: &[&str]) -> Diode {
        let i_sat = 1e-9;
        let tdegc = 27.0;
        let _ = extract_identifier(bits[0]);
        let node1 = self.extract_node(bits[1]);
        let node2 = self.extract_node(bits[2]);
        //let value = extract_value(&bits[3]);

        #[allow(clippy::manual_range_contains)]
        if (i_sat < 0.0) || (i_sat > 1e-6) {
            println!("*WARN* check diode saturation current. It seems weird");
        }
        Diode::new(bits[0], node1, node2, i_sat, tdegc)
    }


    // only support one parameter per option line
    /// Parse a control block option command
    fn extract_option(&mut self, bits: &[&str]) {

        // just 'option' with no arguments? - print the options as they stand
        if bits.len() == 1 {
            self.cfg.print_options();
            return;
        }

        // FIXME - might be "abstol=2e6" too - lower case, no spaces surrounding '='
        if bits[2] != "=" {
            println!("*ERROR* Expected '=' in option setting");
            return;
        }

        match bits[1] {
            "ABSTOL" => {
                self.cfg.ABSTOL = extract_value(bits[3]).unwrap();
            },
            "RELTOL" => {
                self.cfg.RELTOL = extract_value(bits[3]).unwrap();
            },
            _ => {
            }
        }
    }

    /// Parse a Sine Current Source description from SPICE
    fn extract_i_sine(&mut self, bits: &[&str]) -> CurrentSourceSine {
        let _ = extract_identifier(bits[0]);
        let node1 = self.extract_node(bits[1]);
        let node2 = self.extract_node(bits[2]);

        // ugly stuff...
        // push all the remaining bits of the SPICE line into 1 string
        let mut line = "".to_string();
        for b in bits[3..].iter() {
            line += *b;
            line.push(' ');
        }

        // then when we remove "SIN" "(" and ")" we should be left with
        // some numbers that we can extract
        line = line.replace("SIN", "");
        line = line.replace('(', "");
        line = line.replace(')', "");

        let all_bits :Vec<&str> = line.split_whitespace().collect();
        if all_bits.len() != 3 {
            println!("*ERROR* not enough parameters to SIN()");
        }
        let offset = extract_value(all_bits[0]).unwrap();
        let amplitude = extract_value(all_bits[1]).unwrap();
        let frequency = extract_value(all_bits[2]).unwrap();
        trace!("*INFO* ISIN {} {} {}", offset, amplitude, frequency);

        CurrentSourceSine {
            p: node1,
            n: node2,
            vo: offset,
            va: amplitude,
            freq: frequency,
        }
    }

    // extract the stuff from SIN()
    fn extract_v_sine(&mut self, bits: &[&str]) -> VoltageSourceSine {
        let _ = extract_identifier(bits[0]);
        let node1 = self.extract_node(bits[1]);
        let node2 = self.extract_node(bits[2]);

        // ugly stuff...
        // push all the remaining bits of the SPICE line into 1 string
        let mut line = "".to_string();
        for b in bits[3..].iter() {
            line += *b;
            line.push(' ');
        }

        // then when we remove "SIN" "(" and ")" we should be left with
        // some numbers that we can extract
        line = line.replace("SIN", "");
        line = line.replace('(', "");
        line = line.replace(')', "");

        let all_bits :Vec<&str> = line.split_whitespace().collect();
        if all_bits.len() != 3 {
            println!("*ERROR* not enough parameters to SIN()");
        }
        let offset = extract_value(all_bits[0]).unwrap();
        let amplitude = extract_value(all_bits[1]).unwrap();
        let frequency = extract_value(all_bits[2]).unwrap();
        trace!("*INFO* VSIN {} {} {}", offset, amplitude, frequency);

        VoltageSourceSine {
            p: node1,
            n: node2,
            vo: offset,
            va: amplitude,
            freq: frequency,
            idx: 0
        }
    }


    /// Parse an instantiation line
    pub fn extract_instance(&mut self,  bits: &[&str]) -> Instance {
        // If we're here, we know bits[0] starts with 'X'
        // fuckit, we'll just leave the x in the inst name...
        let ident = bits[0];
        let subckt = bits[bits.len()-1]; // last identifier is a name

        let mut inst = Instance::new(ident, subckt);

        // Store the connections as NodeIds
        for conn in bits.iter().take(bits.len()-1).skip(1) {
            let nid = self.ckts[self.c].add_node(conn);
            inst.add_connection(nid);
        }
        trace!("{}", inst);
        inst
    }


    /// Nodes are integers (for now)
    ///
    /// This:
    /// * Parses the node name from the SPICE file
    ///   * If the parsing fails, returns `None`.
    /// * Looks the node name up in the nodelist
    ///   * if it doesn't exist, creates an entry for it and assigns a NodeId
    ///   * if it exists, gets the NodeId
    ///
    /// Ground aka `0`, `gnd` or `GND` is a global net.
    fn extract_node(&mut self, text: &str) -> usize {

        // is this a well-formed node name?
        let mut well_formed_node_name = true;
        for c in text.chars() {
            match c {
                '_' | '0'..='9' | 'a'..='z' | 'A'..='Z' => {},
                _ => { well_formed_node_name = false }
            }
        }
        if !well_formed_node_name {
                println!("*ERROR* bad node name: '{}'", text);
                self.there_are_errors = true;
                return 0;
        }

        self.ckts[self.c].add_node(text)
    }

    /// Find the index of the subcircuit called `name`.
    pub fn find_subckt_index(&self, name: &str) -> Option<usize> {
        for (i, ckt) in self.ckts.iter().enumerate() {
            if ckt.name == name {
                return Some(i);
            }
        }
        None
    }

    /// Return reference to the completed circuit datastructures
    /// Should I create the toplevel here?
    /// I think I should create the toplevel here...
    // I can't figure out how to implement `Copy`, so can I update
    // the toplevel circuit directly? I needed to make things `Clone`.
    pub fn circuit(&self) -> Circuit {

        let mut ckt = self.ckts[0].clone();
        let mut subckt_id = 0;
        let mut hier: Vec<String> = vec![];

        println!("-- Deal with subcircuits -----------------------");
        let insts: &Vec<Instance> = &ckt.instances.clone();
        for inst in insts {
            println!("{}", inst);
            hier.push(inst.name.to_string());

            // find the subckt definition index
            if let Some(ckt_id) = self.find_subckt_index(&inst.subckt) {
                trace!("Found definition for {}", inst.subckt);
                subckt_id = ckt_id;
            } else {
                println!("*ERROR* Can't find a definition for subcircuit {}",
                    inst.subckt);
            }

            // check that the instantiation and the subckt agree on the
            // number of ports
            //dbg!(self.ckts[subckt_id].num_ports, inst.conns.len());
            if self.ckts[subckt_id].num_ports != inst.conns.len() {
                print!("*ERROR* Instantiation and subcircuit definitions");
                println!("have different port sizes");
            }

            trace!("Subcircuit index: {}", subckt_id);

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
            for subckt_el in &self.ckts[subckt_id].elements {
                    trace!(" Element: {}", subckt_el);

                    match subckt_el {

                        Element::R(subckt_res) => {
                            trace!("Found a resistor subcircuit element");

                            // Copy R, cos we have to tweak the nodeids for its ports
                            let mut res = subckt_res.clone();
                            hier.push(res.ident);
                            res.ident = hier.join(".");
                            hier.pop();

                            if subckt_res.a > self.ckts[subckt_id].num_ports {
                                let local_node_name = &self.ckts[subckt_id].node_id_lut[&subckt_res.a];
                                hier.push(local_node_name.to_string());
                                let node_name = hier.join(".");
                                let nid = ckt.add_node(&node_name);
                                res.a = nid;
                                _ = hier.pop();
                                "internal node, so replicate"
                            } else {
                                // we have a port
                                // Find out what node it is connected to above
                                // ports are added first to subckts, so they have NodeIds of 1..P
                                // where there are P ports.
                                // We can use the instantiation to get at the NodeId. 
                                let nid = inst.conns[subckt_res.a-1];
                                res.a = nid;
                                "port node, look up in circuit above"
                            };

                            if subckt_res.b > self.ckts[subckt_id].num_ports {
                                let local_node_name = &self.ckts[subckt_id].node_id_lut[&subckt_res.b];
                                hier.push(local_node_name.to_string());
                                let node_name = hier.join(".");
                                let nid = ckt.add_node(&node_name);
                                res.b = nid;
                                _ = hier.pop();
                            } else {
                                let nid = inst.conns[subckt_res.b-1];
                                res.b = nid;
                            };

                            ckt.elements.push(Element::R(res));
                        },

                        Element::C(subckt_cap) => {
                            trace!("Found a capacitor subcircuit element");

                            // Copy element, cos we have to tweak the nodeids for its ports
                            let mut cap = subckt_cap.clone();
                            hier.push(cap.ident);
                            cap.ident = hier.join(".");
                            hier.pop();

                            if subckt_cap.a > self.ckts[subckt_id].num_ports {
                                let local_node_name = &self.ckts[subckt_id].node_id_lut[&subckt_cap.a];
                                hier.push(local_node_name.to_string());
                                let node_name = hier.join(".");
                                let nid = ckt.add_node(&node_name);
                                cap.a = nid;
                                _ = hier.pop();
                            } else {
                                // we have a port
                                // Find out what node it is connected to above
                                // ports are added first to subckts, so they have NodeIds of 1..P
                                // where there are P ports.
                                // We can use the instantiation to get at the NodeId. 
                                let nid = inst.conns[subckt_cap.a-1];
                                cap.a = nid;
                            };

                            if subckt_cap.b > self.ckts[subckt_id].num_ports {
                                let local_node_name = &self.ckts[subckt_id].node_id_lut[&subckt_cap.b];
                                hier.push(local_node_name.to_string());
                                let node_name = hier.join(".");
                                let nid = ckt.add_node(&node_name);
                                cap.b = nid;
                                _ = hier.pop();
                            } else {
                                let nid = inst.conns[subckt_cap.b-1];
                                cap.b = nid;
                            };

                            ckt.elements.push(Element::C(cap));
                        },

                        Element::D(subckt_diode) => {
                            trace!("Found a diode subcircuit element");

                            // Copy R, cos we have to tweak the nodeids for its ports
                            let mut diode = subckt_diode.clone();
                            hier.push(diode.ident);
                            diode.ident = hier.join(".");
                            hier.pop();

                            if subckt_diode.p > self.ckts[subckt_id].num_ports {
                                let local_node_name = &self.ckts[subckt_id].node_id_lut[&subckt_diode.p];
                                hier.push(local_node_name.to_string());
                                let node_name = hier.join(".");
                                let nid = ckt.add_node(&node_name);
                                diode.p = nid;
                                _ = hier.pop();
                            } else {
                                // we have a port
                                // Find out what node it is connected to above
                                // ports are added first to subckts, so they have NodeIds of 1..P
                                // where there are P ports.
                                // We can use the instantiation to get at the NodeId. 
                                let nid = inst.conns[subckt_diode.p-1];
                                diode.p = nid;
                            };

                            if subckt_diode.n > self.ckts[subckt_id].num_ports {
                                let local_node_name = &self.ckts[subckt_id].node_id_lut[&subckt_diode.n];
                                hier.push(local_node_name.to_string());
                                let node_name = hier.join(".");
                                let nid = ckt.add_node(&node_name);
                                diode.n = nid;
                                _ = hier.pop();
                            } else {
                                let nid = inst.conns[subckt_diode.n-1];
                                diode.n = nid;
                            };

                            ckt.elements.push(Element::D(diode));
                        },


                        _ => {},
                    }

            }
            _ = hier.pop();
        }
        println!("------------------------------------------------");
        ckt.build_node_id_lut();
        ckt
    }

    /// Return reference to the completed configuration object
    pub fn configuration(&self) -> &Configuration {
        &self.cfg
    }

}

/// Extract an element identifier from SPICE
// Just take the entire thing as an identifier
fn extract_identifier(text: &str) -> String {
    text.to_string()
}


#[derive(Debug)]
enum ValueState {
    Start,
    Int,
    Frac,
    ExpStart, // '+' | '-' | digit
    Exp, // digit
    Unit,
}

// possibilities:
// * 10
// * 10.0
// * 10.0m
// * 10.0meg [not implemented]
// * 10mA
// * 10.0megV [not implemented]
// * 10.0e-6 [not implemented]
// * 10.0e-6V [not implemented]
//
// Supported engineering: k m u n p (future: meg f)
// Supported units: NONE (future:  A V F s)
fn extract_value(text: &str) -> Option<f64> {
    let mut value: Option<f64> = None;
    let mut float_str = "".to_string();
    let mut c: char;
    let mut state = ValueState::Start;
    let mut nxt;
    let mut eng_mult :f64 = 1.0;

    //println!("VALUE: '{}'", text);
    let mut text_iter = text.chars();

    fn eval( txt :&str, mult: f64) -> Option<f64> {
        Some( txt.parse::<f64>().unwrap() * mult )
    }

    'things: loop {

        if let Some(c_) = text_iter.next() {
            c = c_;
        } else {
            break 'things;
        }
        //println!(" {:?} '{}'", state, c);
        match state {

            ValueState::Start => {
                match c {
                    '+' | '-' => { float_str.push(c); nxt = ValueState::Int },
                    '0' ..= '9' => { float_str.push(c); nxt = ValueState::Int },
                    _ => break 'things
                }
            },

            ValueState::Int => {
                match c {
                    '0' ..= '9' => { float_str.push(c); nxt = ValueState::Int },
                    '.' => { float_str.push(c); nxt = ValueState::Frac },
                    'e' => { float_str.push(c); nxt = ValueState::ExpStart },
                    'k' => {
                        eng_mult = 1e3;
                        value = eval(&float_str, eng_mult);
                        nxt = ValueState::Unit
                    },
                    'm' => {
                        eng_mult = 1e-3;
                        value = eval(&float_str, eng_mult);
                        nxt = ValueState::Unit
                    },
                    'u' => {
                        eng_mult = 1e-6;
                        value = eval(&float_str, eng_mult);
                        nxt = ValueState::Unit
                    },
                    'n' => {
                        eng_mult = 1e-9;
                        value = eval(&float_str, eng_mult);
                        nxt = ValueState::Unit
                    },
                    'p' => {
                        eng_mult = 1e-12;
                        value = eval(&float_str, eng_mult);
                        nxt = ValueState::Unit
                    },
                    _ => break 'things
                }
            },

            ValueState::Frac => {
                match c {
                    '0' ..= '9' => { float_str.push(c); nxt = ValueState::Frac },
                    'e' => { float_str.push(c); nxt = ValueState::ExpStart },
                    'k' => {
                        eng_mult = 1e3;
                        value = eval(&float_str, eng_mult);
                        nxt = ValueState::Unit
                    },
                    'm' => {
                        eng_mult = 1e-3;
                        value = eval(&float_str, eng_mult);
                        nxt = ValueState::Unit
                    },
                    'u' => {
                        eng_mult = 1e-6;
                        value = eval(&float_str, eng_mult);
                        nxt = ValueState::Unit
                    },
                    'n' => {
                        eng_mult = 1e-9;
                        value = eval(&float_str, eng_mult);
                        nxt = ValueState::Unit
                    },
                    'p' => {
                        eng_mult = 1e-12;
                        value = eval(&float_str, eng_mult);
                        nxt = ValueState::Unit
                    },
                    _ => break 'things
                }
            },

            ValueState::ExpStart => {
                match c {
                    '+' | '-' => { float_str.push(c); nxt = ValueState::Exp },
                    '0' ..= '9' => { float_str.push(c); nxt = ValueState::Exp },
                    _ => break 'things
                }
            },

            ValueState::Exp => {
                match c {
                    '0' ..= '9' => { float_str.push(c); nxt = ValueState::Exp },
                    _ => break 'things
                }
            },

            ValueState::Unit => {
                break 'things
            },
        }

        //println!(" -> {:?} '{}'", nxt, float_str);
        state = nxt;
    }

    // if we've broken out of the loop at a point where the gathered
    // string might be a valid number, calculate it.
    match state {
        ValueState::Int | ValueState::Frac | ValueState::Exp => {
            value = eval(&float_str, eng_mult)
        },
        _ => {}
    }

    value
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_read_count_voltage_sources() {
        let mut rdr = Reader::new();
        rdr.read(Path::new("./ngspice/test_reader.spi"));
        assert!(rdr.ckts[0].count_voltage_sources() == 1);
    }

    #[test]
    fn simple_read_count_nodes() {
        let mut rdr = Reader::new();
        rdr.read(Path::new("./ngspice/test_reader.spi"));
        assert!(rdr.ckts[0].count_nodes() == 9);
    }

    #[test]
    fn simple_subckt_counts() {
        let mut rdr = Reader::new();
        rdr.read(Path::new("./ngspice/subckt_fullwave_rectifier.spi"));
        assert!(rdr.ckts.len() == 3);

        assert!(rdr.ckts[0].nodes.len() == 5);
        assert!(rdr.ckts[0].instances.len() == 2);

        // bridge
        assert!(rdr.ckts[1].nodes.len() == 5);
        assert!(rdr.ckts[1].instances.len() == 0);

        // load
        assert!(rdr.ckts[2].nodes.len() == 6);
        assert!(rdr.ckts[2].instances.len() == 0);

        // elaborated circuit
        let ckt = rdr.circuit();
        assert!(ckt.nodes.len() == 8);
    }
}
