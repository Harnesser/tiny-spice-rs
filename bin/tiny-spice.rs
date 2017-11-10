extern crate tiny_spice;

use std::env;

use tiny_spice::spice;
use tiny_spice::engine;

use tiny_spice::analysis::{Kind};

/// Read a spice file, and execute it
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please supply a SPICE filename");
        panic!("adsfasDSF");
    }

    let mut reader = spice::Reader::new();
    reader.read(&args[1]);

    let ckt = reader.circuit();
    let mut cfg = reader.configuration();

    /*
    let commands = reader.commands();
    for cmd in commands {
        let mut eng = engine::Engine::new();
        match cmd {
            //OP => eng.dc_operating_point(rdr.circuit()),
            TRANS => eng.transient_analysis(rdr.circuit()),
            _ => panic!("Unimplemented command"),
        }
    }
*/

    // tmp analysis
    let mut eng = engine::Engine::new();
    if let Some(stats) = eng.go(&ckt, &cfg) {
        println!("\n*INFO* Done");
    } else {
        println!("\n*ERROR* Bad, bad bad...");
    }

    if false {
        let mut cfg2 = cfg.clone();
        cfg2.set_transient(3e-3, 1e-6, 0.0);
        cfg2.set_wavefile("waves/asdfasdf.dat");
        eng.go(&ckt, &cfg2);
    }
}

