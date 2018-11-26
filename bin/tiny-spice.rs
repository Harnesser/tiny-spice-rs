extern crate tiny_spice;

use std::env;

use tiny_spice::spice;
use tiny_spice::engine;

use tiny_spice::analysis::{Kind};

/// Read a spice file, and execute it
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please supply a SPICE filename");
    }

    let mut reader = spice::Reader::new();
    reader.read(&args[1]);

    let ckt = reader.circuit();
    let cfg = reader.configuration();

    // tmp analysis
    let mut eng = engine::Engine::new();
    if let Some(stats) = eng.go(&ckt, &cfg) {
        println!("\n*INFO* Done");
    } else {
        println!("\n*ERROR* Bad, bad bad... '{}'", &args[1]);
    }

}

