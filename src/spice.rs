//! Read a SPICE Deck
//!
//! Tiny-Spice-rs supports reading the the following circuit things from a
//! SPICE deck description of a circuit.
//!
//! * Initial comment line
//! * Components:
//!   * Voltage source : `V<ident> <n+> <n-> <value>`
//!   * Current source : `I<ident> <n+> <n-> <value>`
//!   * Resistor : `R<ident> <n1> <n2> <value>`
//! * Node names:
//!   * Integers (e.g. `23`) or text (e.g. `node_3452`)
//!   * As usual, `0` , `gnd` and `GND` are aliases
//! * Values:
//!   * Numbers (e.g `400` or `420.69`
//!   * Numbers with optional engineering scaler (e.g. `10k` or `10.5u`)
//! * Control Blocks:
//!   Only one operation for now - no sequences
//!   * DC Operating Point `op`
//!   * Transient : `tran <t_step> <t_stop> [t_start]`
//!     * for now, `t_start` is ignored
//! * Options (in Control Blocks)
//!   * Options: `option <OPTION_NAME> = <value>`
//!     * `ABSTOL`
//!     * `RELTOL`
//!     * `RMAX`
//! * Subcircuits
//!   * Subcircuit definitions with `.subckt` and `.ends`
//!   * Instantiations with `X...`

use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};

use crate::circuit::{Circuit, CurrentSourceSine, VoltageSourceSine};
use crate::circuit::{VoltageSourcePwl};
use crate::circuit::{Instance};
use crate::parameter::{Parameter};
use crate::bracket_expression::{extract_expression, extract_value};
use crate::bracket_expression::Expression::{Literal};

use crate::analysis::{Configuration, Kind};
use crate::expander;

/// Program execution trace macro - prefix `<spice>`
macro_rules! trace {
    ($fmt:expr $(, $($arg:tt)*)?) => {
        // uncomment the line below for tracing prints
        //println!(concat!("<spice> ", $fmt), $($($arg)*)?);
    };
}


/// Datastructure for info parsed from the SPICE deck
pub struct Reader {
    /// Circuit information
    /// `ckts[0]` is the toplevel
    ckts: Vec<Circuit>,
    /// Options and analysis commands
    cfg: Configuration,
    /// Track which circuit in the `ckts` list that we're adding things too
    c: usize,
    /// Flag if problems were encountered during parsing
    there_are_errors: bool
}


impl Reader {

    #[allow(clippy::new_without_default)]
    /// Initialise with an empty toplevel circuit
    pub fn new() -> Reader {
        let topckt = Circuit::new();
        Reader {
            ckts: vec![topckt],
            cfg: Configuration::new(),
            c: 0, // toplevel
            there_are_errors: false,
        }
    }

    /// Read and parse a SPICE deck
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
                    } else if bits[3].starts_with("PWL(") {
                        trace!("*INFO* Vpwl");
                        let src = self.extract_v_pwl(&bits);
                        self.ckts[self.c].add_v_pwl(src);
                    } else {
                        println!("Some weird V thing you've got going here");
                        println!("bits: {:?}", bits);
                    }
                } else if bits[0].starts_with('R') {
                    if let Some(r) = self.extract_primitive(&bits, 2, 1) {
                        self.ckts[self.c].add_instance(r);
                    } else {
                        println!("*ERROR* can't extract resistance");
                    }
                } else if bits[0].starts_with('C') {
                    if let Some(c) = self.extract_primitive(&bits, 2, 1) {
                        self.ckts[self.c].add_instance(c);
                    } else {
                        println!("*ERROR* can't extract capacitor");
                    }
                } else if bits[0].starts_with('D') {
                    if let Some(d) = self.extract_primitive(&bits, 2, 0) {
                        self.ckts[self.c].add_instance(d);
                    } else {
                        println!("*ERROR* can't extract diode");
                    }
                } else if bits[0].starts_with('E') {
                    if let Some(e) = self.extract_primitive(&bits, 4, 1) {
                        self.ckts[self.c].add_instance(e);
                    } else {
                        println!("*ERROR* can't extract VCVS");
                    }
                } else if bits[0].starts_with('G') {
                    if let Some(g) = self.extract_primitive(&bits, 4, 1) {
                        self.ckts[self.c].add_instance(g);
                    } else {
                        println!("*ERROR* can't extract VCCS");
                    }
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
                            if nn.contains('=') {
                                trace!("Found parameter: {}", nn);
                                if let Some(param) = self.extract_parameter(nn) {
                                    self.ckts[self.c].params.push(param);
                                } else {
                                    println!("*ERROR* can't extract parameter");
                                    self.there_are_errors = true;
                                }
                                continue;
                            }
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
                } else {
                    panic!("Unrecognised circuit element: '{}'", bits[0]);
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
            ckt.list_parameters();
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
            "RMAX" => {
                self.cfg.RMAX = extract_value(bits[3]).unwrap();
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


    /// extract the stuff from PWL
    fn extract_v_pwl(&mut self, bits: &[&str]) -> VoltageSourcePwl {
        let _ = extract_identifier(bits[0]);
        let node1 = self.extract_node(bits[1]);
        let node2 = self.extract_node(bits[2]);

        let mut src = VoltageSourcePwl {
            p: node1,
            n: node2,
            pat: vec![],
            repeat: -1.0,
            t_delay: 0.0,
            idx: 0
        };

        // work from the end and pick off any variables:
        // in this case, delay and repeat time
        let mut n_params = 0;
        for i in (0..bits.len()).rev() {
            if bits[i].contains('=') {
                n_params += 1;
                if let Some(param) = self.extract_override(bits[i]) {

                    // eugh... assume the parameter value is a literal.
                    let val = if let Some(Literal(v)) = param.expr {
                        v
                    } else {
                        0.0
                    };

                    match param.name.as_str() {
                        "r" => { src.repeat = val},
                        "td" => { src.t_delay = val},
                        _ => {
                            println!("*WARNING* unrecogised PWL parameter '{}'",
                                param.name);
                        }
                    }
                } else {
                    println!("*WARNING* something bad in VPWL");
                }
            } else {
                break;
            }
        }

        // ugly stuff...
        // push all the remaining bits of the SPICE line into 1 string
        let mut line = "".to_string();
        let end = bits.len() - n_params;
        for b in bits[3..end].iter() {
            line += *b;
            line.push(' ');
        }

        // then when we remove "PWL" "(" and ")" we should be left with
        // some numbers that we can extract
        line = line.replace("PWL", "");
        line = line.replace('(', "");
        line = line.replace(')', "");
        //trace!("Line: {}", line);
        //println!("Line: {}", line);

        let all_bits: Vec<&str> = line.split(',').collect();

        let mut tvs: Vec<(f64, f64)> = vec![];

        for tv_pair in all_bits.chunks(2) {
            if tv_pair.len() == 1 {
                println!("*WARNING* Unmatched time value: {}", tv_pair[0]);
            }
            let t = extract_value(tv_pair[0].trim()).unwrap();
            let v = extract_value(tv_pair[1].trim()).unwrap();
            tvs.push((t,v));
            trace!("time: {:?}, value: {:?}", t, v);
        }
        src.pat = tvs;

        //trace!("*INFO* VPWL {} {} {}", offset, amplitude, frequency);
        trace!("{:?}", src);
        src
    }


    /// Parse an instantiation line
    ///
    /// 2nd last non-`<ident>=<value>` bit is the subcircuit name
    pub fn extract_instance(&mut self,  bits: &[&str]) -> Instance {
        // If we're here, we know bits[0] starts with 'X'
        // fuckit, we'll just leave the x in the inst name...
        let ident = bits[0];

        let mut inst = Instance::new(ident, "(not-found)");

        // Paramters, if there are any
        // Search back from the end of the `bits` list until we find the first
        // text section without an "=" sign. This is the subcircuit name.
        // This all assumes that there are no spaces around the equals sign in
        // "<ident>=<expr>".
        let mut subckt_id = 0;
        for i in (0..bits.len()).rev() {
            if bits[i].contains('=') {
                if let Some(param) = self.extract_override(bits[i]) {
                    inst.add_parameter(&param);
                } else {
                    println!("*ERROR* paraeter in instance bad");
                    self.there_are_errors = true;
                }
            } else {
                subckt_id = i;
                break;
            }
        }
        inst.subckt = bits[subckt_id].to_string();

        // Store the connections as NodeIds
        for conn in bits.iter().take(subckt_id).skip(1) {
            let nid = self.ckts[self.c].add_node(conn);
            inst.add_connection(nid);
        }

        trace!("{}", inst);
        inst
    }


    /// Extract a SPICE node identifier from a lump text
    ///
    /// Node identifiers can be integers or strings, e.g. both `69` and
    /// `bridge_output_234234` are valid node names
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

    /// Extract a parameter definition from a `.subckt` line
    ///
    /// The value goes in the `defval` field.
    pub fn extract_parameter(&mut self, text: &str) -> Option<Parameter> {
        // mut is for `there are errors`
        let bits: Vec<_> = text.split('=').collect();
        if bits.len() != 2 {
            println!("*ERROR* expected <ident>=<expr>");
            self.there_are_errors = true;
            return None
        }

        let name = extract_identifier(bits[0]);
        if let Some(expr) = extract_expression(bits[1]) {
            Some(Parameter::default_from_expression(&name, &expr))
        } else {
            println!("*ERROR* expected <ident>=<expr>");
            self.there_are_errors = true;
            None
        }
    }

    /// Extract a parameter definition from an instantiation line
    ///
    /// The value goes in the `expr` field.
    pub fn extract_override(&mut self, text: &str) -> Option<Parameter> {
        // mut is for `there are errors`
        let bits: Vec<_> = text.split('=').collect();
        if bits.len() != 2 {
            println!("*ERROR* expected <ident>=<expr>");
            self.there_are_errors = true;
            return None
        }

        let name = extract_identifier(bits[0]);
        if let Some(expr) = extract_expression(bits[1]) {
            Some(Parameter::override_from_expression(&name, &expr))
        } else {
            println!("*ERROR* expected <ident>=<expr>");
            self.there_are_errors = true;
            None
        }
    }

    /// Extract an instantiation of a device primitive
    pub fn extract_primitive(
        &mut self,
        bits: &[&str],
        num_ports: usize,
        min_num_params: usize
    )
        -> Option<Instance>
    {
        trace!("extracting primitive device");

        if bits.len() < (1 + num_ports + min_num_params) {
            println!("*ERROR* not enough bits");
            return None
        }

        let ident = extract_identifier(bits[0]);
        let mut inst = Instance::new(&ident, r"/device");
        let mut i = 1;

        for _j in 0..num_ports {
            let node = self.extract_node(bits[i]);
            inst.conns.push(node);
            i += 1;
        }

        // parameters
        // going by the calculations above, there should be a minimum number
        // of parameters we expect to parse
        for (p, param_text) in bits.iter().skip(i).enumerate() {
            let name = format!("/param{}", p);
            if let Some(expr) = extract_expression(param_text) {
                let param = Parameter::override_from_expression(&name, &expr);
                inst.params.push(param);
            } else {
                println!("*WARN* or maybe an err, i dunno");
            }
        }

        trace!("Primitive: {}", inst);

        Some(inst)
    }

    /// Return a circuit that is the expansion of the toplevel circuit
    ///
    /// The toplevel circuit instantiations are resolved with all subcircuit
    /// elements hoisted up into a clone of the toplevel circuit.
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

#[cfg(test)]
mod tests {

    // how do I use the one from tests::common?
    pub fn assert_nearly(x: f64, expected: f64) {
        const EPSILON: f64 = 1e-9;
        let delta = (x - expected).abs();
        assert!( delta < EPSILON, "{} isn't approximately {}", x, expected);
    }

    // If node count for the expanded circuit goes wrong and the SPICE file hasn't
    // changed, it's usually the hierarchy stack that's passed around not getting
    // pushed or popped symmetrically in the Expander.

    use super::*;

    #[test]
    fn simple_read_count_voltage_sources() {
        let mut rdr = Reader::new();
        rdr.read(Path::new("./ngspice/test_reader.spi"));
        assert_eq!(rdr.ckts[0].count_voltage_sources(), 1);
    }

    #[test]
    fn simple_read_count_nodes() {
        let mut rdr = Reader::new();
        rdr.read(Path::new("./ngspice/test_reader.spi"));
//        assert_eq!(rdr.ckts[0].count_nodes(), 9); //  FIXME
        assert_eq!(rdr.ckts[0].nodes.len(), 9);
    }

    #[test]
    fn simple_subckt_counts() {
        let mut rdr = Reader::new();
        rdr.read(Path::new("./ngspice/subckt_fullwave_rectifier.spi"));
        assert_eq!(rdr.ckts.len(), 3);

        assert_eq!(rdr.ckts[0].nodes.len(), 5);
        assert_eq!(rdr.ckts[0].instances.len(), 2);

        // bridge
        assert_eq!(rdr.ckts[1].nodes.len(), 5);
        assert_eq!(rdr.ckts[1].instances.len(), 8); // 4 diodes, 4 caps

        // load
        assert_eq!(rdr.ckts[2].nodes.len(), 6); // 2 ports, 3 internal + gnd
        assert_eq!(rdr.ckts[2].instances.len(), 5); // 4 res + 1 cap

        // elaborated circuit
        let ckt = rdr.get_expanded_circuit();
        ckt.list_elements();
        ckt.list_nodes();
        //assert_eq!(ckt.nodes.len(), 8); // this include aliases...
        assert_eq!(ckt.node_id_lut.len(), 8); // this include aliases...
    }

    #[test]
    fn multilevel_subckt_counts() {
        let mut rdr = Reader::new();
        rdr.read(Path::new("./ngspice/multilevel_subckt_fullwave_rectifier.spi"));
        assert_eq!(rdr.ckts.len(), 4);

        assert_eq!(rdr.ckts[0].nodes.len(), 10); // gnd + 3 op pairs + 3 v stack
        assert_eq!(rdr.ckts[0].instances.len(), 6); // 3x (system + cap)

        // bridge
        assert_eq!(rdr.ckts[1].nodes.len(), 5);
        assert_eq!(rdr.ckts[1].instances.len(), 8); // 4x diode, 4x cap

        // system
        assert_eq!(rdr.ckts[2].nodes.len(), 6); // 4x port, 1x internal, gnd
        assert_eq!(rdr.ckts[2].instances.len(), 3); // bridge, R, rload

        // load
        assert_eq!(rdr.ckts[3].nodes.len(), 6); // 2x port, 3x internal, gnd
        assert_eq!(rdr.ckts[3].instances.len(), 4); // 4x R

        // elaborated circuit
        let ckt = rdr.get_expanded_circuit();
//      assert_eq!(ckt.nodes.len(), 8); // this has aliase
        assert_eq!(ckt.node_id_lut.len(), 22); // HARDCODED! 6x 3 + 4

    }

    #[test]
    fn param_subckt_counts() {
        let mut rdr = Reader::new();
        rdr.read(Path::new("./ngspice/param_fullwave_rectifier.spi"));
        assert_eq!(rdr.ckts.len(), 4);

        assert_eq!(rdr.ckts[0].nodes.len(), 10); // gnd + 3 op pairs + 3 v stack
        assert_eq!(rdr.ckts[0].instances.len(), 3); // 3x (system + cap)
        assert_eq!(rdr.ckts[0].params.len(), 0);

        // bridge
        assert_eq!(rdr.ckts[1].nodes.len(), 5);
        assert_eq!(rdr.ckts[1].instances.len(), 8); // 4x diode, 4x cap
        assert_eq!(rdr.ckts[1].params.len(), 0);

        // system
        assert_eq!(rdr.ckts[2].nodes.len(), 6); // 4x port, 1x internal, gnd
        assert_eq!(rdr.ckts[2].instances.len(), 3); // bridge, R, rload
        assert_eq!(rdr.ckts[2].params.len(), 1); // cval

        // load
        assert_eq!(rdr.ckts[3].nodes.len(), 6); // 2x port, 3x internal, gnd
        assert_eq!(rdr.ckts[3].instances.len(), 5); // 4x R + 1 cap
        assert_eq!(rdr.ckts[3].params.len(), 1); // cvalo

        // elaborated circuit
        let ckt = rdr.get_expanded_circuit();
//      assert_eq!(ckt.nodes.len(), 8); // this has aliase
        assert_eq!(ckt.node_id_lut.len(), 22); // HARDCODED! 6x 3 + 4

        // check if the parameters propagated...
        assert_nearly(ckt.get_param_value("Xsystem1.Xload.cvalo").unwrap(),   1e-6);
        assert_nearly(ckt.get_param_value("Xsystem2.Xload.cvalo").unwrap(),  10e-6);
        assert_nearly(ckt.get_param_value("Xsystem3.Xload.cvalo").unwrap(), 100e-6);
    }

    #[test]
    fn v_pwl() {
        let mut rdr = Reader::new();
        rdr.read(Path::new("./ngspice/v_pwl.spi"));
        assert_eq!(rdr.ckts.len(), 1);

        assert_eq!(rdr.ckts[0].nodes.len(), 5); 
        // voltage sources don't count yet
        assert_eq!(rdr.ckts[0].instances.len(), 0);
        assert_eq!(rdr.ckts[0].elements.len(), 4);
        assert_eq!(rdr.ckts[0].params.len(), 0);

        // elaborated circuit
        let ckt = rdr.get_expanded_circuit();
        assert_eq!(ckt.node_id_lut.len(), 5);
    }

    #[test]
    fn vcvs_vccs() {
        let mut rdr = Reader::new();
        rdr.read(Path::new("./ngspice/vc_vs_cs.spi"));
        assert_eq!(rdr.ckts.len(), 1);

        assert_eq!(rdr.ckts[0].nodes.len(), 4); 
        assert_eq!(rdr.ckts[0].instances.len(), 3); // E, G & R
        assert_eq!(rdr.ckts[0].elements.len(), 1); // Vsim
        assert_eq!(rdr.ckts[0].params.len(), 0);

        // elaborated circuit
        let ckt = rdr.get_expanded_circuit();
        assert_eq!(ckt.node_id_lut.len(), 4);
    }


}
