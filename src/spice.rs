//! Read a SPICE Deck 
//!
//! Supported:
//! 1. Initial comment line
//! 2. Components:
//!   * Voltage source : V<ident> <n+> <n-> <value>
//!   * Current source : I<ident> <n+> <n-> <value>
//!   * Resistor : R<ident> <n1> <n2> <value>
//! 3. Node names:
//!   * Integers for now
//! 4. Values:
//!   * floating point with optional engineering scaler and unit

use std::fs::File;
use std::io::{BufReader, BufRead};

use circuit::{Circuit, Diode, CurrentSourceSine};
use analysis::{Configuration, Kind};

pub struct Reader {
    ckt: Circuit,
    cfg: Configuration,
}

impl Reader {

    pub fn new() -> Reader {
        Reader {
            ckt: Circuit::new(),
            cfg: Configuration::new(),
        }
    }

    pub fn read(&mut self, filename :&str) {

        let input = File::open(filename).unwrap();
        let buf = BufReader::new(input);
        let mut lines_iter = buf.lines();
        let mut in_control_block = false;

        // first line is a comment and is ignored
        lines_iter.next();

        for line_wr in lines_iter {
            let line = line_wr.unwrap();
            let all_bits :Vec<&str> = line.split_whitespace().collect();

            // jump blank lines
            if all_bits.len() == 0 {
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

            // let's go
            if in_control_block {
                println!("*INFO* Reading control '{}'", bits[0]);
                if bits[0] == "op" {
                    self.cfg.kind = Some(Kind::DcOperatingPoint);
                } else if bits[0] == "tran" {
                    // !!!FIXME!!! intentionally hobbled until SIN parsing fixed
                    //self.cfg.kind = Some(Kind::Transient);
                    // !!!FIXME!!! intentionally hobbled until SIN parsing fixed
                    // step stop <start>
                    if bits.len() < 3 {
                        println!("*ERROR* not enough trans info");
                    }
                    self.cfg.TSTEP = extract_value(&bits[1]).unwrap();
                    self.cfg.TSTOP = extract_value(&bits[2]).unwrap();
                    if bits.len() > 3 {
                        self.cfg.TSTART = extract_value(&bits[3]).unwrap();
                    }
                }
            } else {

                // find out what we're looking at
                if bits[0].starts_with('I') {
                    let _ = extract_identifier(&bits[0]);
                    let node1 = extract_node(&bits[1]);
                    let node2 = extract_node(&bits[2]);
                    if bits.len() == 4 {
                        println!("*INFO* Idc");
                        let value = extract_value(&bits[3]);
                        self.ckt.add_i(node1, node2, value.unwrap());
                    } else if bits[3].starts_with("SIN") {
                        println!("*INFO* Isin");
                        let src = self.extract_i_sine(&bits);
                        self.ckt.add_i_sin(src);
                    }
                } else if bits[0].starts_with('V') {
                    let _ = extract_identifier(&bits[0]);
                    let node1 = extract_node(&bits[1]);
                    let node2 = extract_node(&bits[2]);
                    if bits.len() == 4 {
                        println!("*INFO* Vdc");
                        let value = extract_value(&bits[3]);
                        self.ckt.add_v(node1, node2, value.unwrap());
                    } else if bits[3] == "SIN(" {
                        println!("*INFO* Vsin");
                    }
                } else if bits[0].starts_with('R') {
                    let _ = extract_identifier(&bits[0]);
                    let node1 = extract_node(&bits[1]);
                    let node2 = extract_node(&bits[2]);
                    let value = extract_value(&bits[3]);
                    self.ckt.add_r(node1, node2, value.unwrap());
                } else if bits[0].starts_with('C') {
                    let _ = extract_identifier(&bits[0]);
                    let node1 = extract_node(&bits[1]);
                    let node2 = extract_node(&bits[2]);
                    let value = extract_value(&bits[3]);
                    self.ckt.add_c(node1, node2, value.unwrap());
                } else if bits[0].starts_with('D') {
                    let d = self.extract_diode(&bits);
                    self.ckt.add_d(d);
                } else if bits[0].starts_with('.') {
                    if bits[0] == ".control" {
                        in_control_block = true;
                    } else if bits[0] == ".endc" {
                        in_control_block = false;
                    } else if bits[0] == ".option" {
                        self.extract_option(&bits);
                    }
                }
            }

            //for bit in bits {
            //    println!("->{}", bit);
            //}

            //println!("{}", line);
        }

    }

    fn extract_diode(&self, bits: &Vec<&str>) -> Diode {
        let mut i_sat = 1e-9;
        let mut tdegc = 27.0;
        let _ = extract_identifier(&bits[0]);
        let node1 = extract_node(&bits[1]);
        let node2 = extract_node(&bits[2]);
        //let value = extract_value(&bits[3]);

        if (i_sat < 0.0) || (i_sat > 1e-6) {
            println!("*WARN* check diode saturation current. It seems weird");
        }
        Diode::new(node1, node2, i_sat, tdegc)
    }

    // only support one parameter per .option line
    fn extract_option(&mut self, bits: &Vec<&str>) {
        if bits[2] != "=" {
            println!("*ERROR* shite....");
        }
        if bits[1] == "ABSTOL" {
            self.cfg.ABSTOL = extract_value(&bits[3]).unwrap();
        }
    }

    // extract the stuff from SIN()
    fn extract_i_sine(&mut self, bits: &Vec<&str>) -> CurrentSourceSine {
        let _ = extract_identifier(&bits[0]);
        let node1 = extract_node(&bits[1]);
        let node2 = extract_node(&bits[2]);

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
        line = line.replace("(", "");
        line = line.replace(")", "");

        let all_bits :Vec<&str> = line.split_whitespace().collect();
        if all_bits.len() != 3 {
            println!("*ERROR* not enough parameters to SIN()");
        }
        let offset = extract_value(all_bits[0]).unwrap();
        let amplitude = extract_value(all_bits[1]).unwrap();
        let frequency = extract_value(all_bits[2]).unwrap();
        println!("*INFO* ISIN {} {} {}", offset, amplitude, frequency);

        CurrentSourceSine {
            p: node1,
            n: node2,
            vo: offset,
            va: amplitude,
            freq: frequency,
        }
    }


    // Return reference to the completed circuit datastructure
    pub fn circuit(&self) -> &Circuit {
        &self.ckt
    }

    // Return reference to the completed configuration object
    pub fn configuration(&self) -> &Configuration {
        &self.cfg
    }

}

// just take the entire thing as an identifier
fn extract_identifier(text: &str) -> String {
    text.to_string()
}

fn extract_node(text: &str) -> usize {
    let mut node: usize = 0;
    match text.parse::<usize>() {
        Ok(n) => node = n,
        Err(_) => println!("*ERROR* bad node name"),
    }
    node
}

#[derive(Debug)]
enum ValueState {
    START,
    INT,
    FRAC,
    UNIT,
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
    let mut state = ValueState::START;
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

            ValueState::START => {
                match c {
                    '+' | '-' => { float_str.push(c); nxt = ValueState::INT },
                    '0' ... '9' => { float_str.push(c); nxt = ValueState::INT },
                    _ => break 'things
                }
            },

            ValueState::INT => {
                match c {
                    '0' ... '9' => { float_str.push(c); nxt = ValueState::INT },
                    '.' => { float_str.push(c); nxt = ValueState::FRAC },
                    'k' => {
                        eng_mult = 1e3;
                        value = eval(&float_str, eng_mult);
                        nxt = ValueState::UNIT
                    },
                    'm' => {
                        eng_mult = 1e-3;
                        value = eval(&float_str, eng_mult);
                        nxt = ValueState::UNIT
                    },
                    'u' => {
                        eng_mult = 1e-6;
                        value = eval(&float_str, eng_mult);
                        nxt = ValueState::UNIT
                    },
                    'n' => {
                        eng_mult = 1e-9;
                        value = eval(&float_str, eng_mult);
                        nxt = ValueState::UNIT
                    },
                    'p' => {
                        eng_mult = 1e-12;
                        value = eval(&float_str, eng_mult);
                        nxt = ValueState::UNIT
                    },
                    _ => break 'things
                }
            },

            ValueState::FRAC => {
                match c {
                    '0' ... '9' => { float_str.push(c); nxt = ValueState::FRAC },
                    'k' => {
                        eng_mult = 1e3;
                        value = eval(&float_str, eng_mult);
                        nxt = ValueState::UNIT
                    },
                    'm' => {
                        eng_mult = 1e-3;
                        value = eval(&float_str, eng_mult);
                        nxt = ValueState::UNIT
                    },
                    'u' => {
                        eng_mult = 1e-6;
                        value = eval(&float_str, eng_mult);
                        nxt = ValueState::UNIT
                    },
                    'n' => {
                        eng_mult = 1e-9;
                        value = eval(&float_str, eng_mult);
                        nxt = ValueState::UNIT
                    },
                    'p' => {
                        eng_mult = 1e-12;
                        value = eval(&float_str, eng_mult);
                        nxt = ValueState::UNIT
                    },
                    _ => break 'things
                }
            },

            ValueState::UNIT => {
                break 'things
            },
        }

        //println!(" -> {:?} '{}'", nxt, float_str);
        state = nxt;
    }

    // if we've broken out of the loop at a point where the gathered
    // string might be a valid number, calculate it.
    match state {
        ValueState::INT | ValueState::FRAC => {
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
    fn simple_read() {
        let mut rdr = Reader::new();
        rdr.read("./ngspice/test_reader.spi");
    }
}
