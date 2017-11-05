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
            println!("found current source");
        } else if bits[0].starts_with('V') {
            println!("found voltage source");
        } else if bits[0].starts_with('R') {
            println!("found resistance");
        }

        for bit in bits {
            println!("->{}", bit);
        }


        println!("{}", line);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_read() {
        read("./ngspice/test_irrc.spi");
    }
}
