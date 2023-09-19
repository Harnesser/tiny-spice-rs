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
use crate::circuit::{Instance};
use crate::analysis::{Configuration, Kind};
use crate::expander;

macro_rules! trace {
    ($fmt:expr $(, $($arg:tt)*)?) => {
        // uncomment the line below for tracing prints
        println!(concat!("<spice> ", $fmt), $($($arg)*)?);
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

    /// Return reference to the completed circuit datastructures
    /// Should I create the toplevel here?
    /// I think I should create the toplevel here...
    // I can't figure out how to implement `Copy`, so can I update
    // the toplevel circuit directly? I needed to make things `Clone`.
    pub fn get_expanded_circuit(&self) -> Circuit {
        expander::expand(&self.ckts)
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
        let ckt = rdr.get_expanded_circuit();
        assert!(ckt.nodes.len() == 8);
    }

    #[test]
    fn multilevel_subckt_counts() {
        let mut rdr = Reader::new();
        rdr.read(Path::new("./ngspice/multilevel_subckt_fullwave_rectifier.spi"));
        assert!(rdr.ckts.len() == 3);

        assert!(rdr.ckts[0].nodes.len() == 5);
        assert!(rdr.ckts[0].instances.len() == 2);

        // bridge
        assert!(rdr.ckts[1].nodes.len() == 5);
        assert!(rdr.ckts[1].instances.len() == 0);

        // load
        assert!(rdr.ckts[2].nodes.len() == 6);
        assert!(rdr.ckts[2].instances.len() == 0);

        // system
        assert!(rdr.ckts[3].nodes.len() == 6);
        assert!(rdr.ckts[3].instances.len() == 0);

        // elaborated circuit
        let ckt = rdr.get_expanded_circuit();
        assert!(ckt.nodes.len() == 8);
    }
}
