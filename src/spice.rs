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

pub fn read(filename :&str) {

    let input = File::open(filename).unwrap();
    let buf = BufReader::new(input);
    let mut lines_iter = buf.lines();

    // first line is a comment and is ignored
    lines_iter.next();

    for line_wr in lines_iter {
        let line = line_wr.unwrap();
        let bits :Vec<&str> = line.split_whitespace().collect();

        // jump blank lines
        if bits.len() == 0 {
            continue;
        }

        // find out what we're looking at
        if bits[0].starts_with('I') {
            println!("\nfound current source");
            let ident = extract_identifier(&bits[0]);
            let node1 = extract_node(&bits[1]);
            let node2 = extract_node(&bits[2]);
            let value = Some(0.0); // extract_value(&bits[3]);
            println!("::: {} {} {} {}", ident, node1, node2, value.unwrap());
        } else if bits[0].starts_with('V') {
            println!("\nfound voltage source");
            let ident = extract_identifier(&bits[0]);
            let node1 = extract_node(&bits[1]);
            let node2 = extract_node(&bits[2]);
            let value = extract_value(&bits[3]);
            println!("::: {} {} {} {}", ident, node1, node2, value.unwrap());
        } else if bits[0].starts_with('R') {
            println!("\nfound resistance: {:?}", bits);
            let ident = extract_identifier(&bits[0]);
            let node1 = extract_node(&bits[1]);
            let node2 = extract_node(&bits[2]);
            let value = extract_value(&bits[3]);
            println!("::: {} {} {} {}", ident, node1, node2, value.unwrap());
        } else if bits[0].starts_with('C') {
            println!("\nfound capacitance");
            let ident = extract_identifier(&bits[0]);
            let node1 = extract_node(&bits[1]);
            let node2 = extract_node(&bits[2]);
            let value = extract_value(&bits[3]);
            println!("::: {} {} {} {}", ident, node1, node2, value.unwrap());
        }

        //for bit in bits {
        //    println!("->{}", bit);
        //}

        println!("{}", line);
    }

}

// just take the entire thing as an identifier
fn extract_identifier(text: &str) -> String {
    text.to_string()
}

fn extract_node(text: &str) -> u32 {
    let mut node: u32 = 0;
    match text.parse::<u32>() {
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
// * 10.0e-6
// * 10.0e-6V
//
// Supported engineering: meg k m u n p f
// Supported units: A V F s
fn extract_value(text: &str) -> Option<f64> {
    let mut value: Option<f64> = None;
    let mut float_str = "".to_string();
    let mut c: char;
    let mut state = ValueState::START;
    let mut nxt;
    let mut eng_mult :f64 = 1.0;

    println!("VALUE: '{}'", text);
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
        println!(" {:?} '{}'", state, c);
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

        println!(" -> {:?} '{}'", nxt, float_str);
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
        read("./ngspice/test_reader.spi");
    }
}
