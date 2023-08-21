extern crate tiny_spice;

use std::env;
use std::path::Path;

use tiny_spice::spice;
use tiny_spice::engine;

use tiny_spice::analysis::{Kind};

/// Read a spice file, and execute it
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please supply a SPICE filename");
    }

    // See if the filename exists
    let spice_file = Path::new(&args[1]);
    spice_file.try_exists().expect("Can't access spice file");

    let mut reader = spice::Reader::new();
    reader.read(&spice_file);

    let ckt = reader.circuit();
    let cfg = reader.configuration();

    // tmp analysis
    let mut eng = engine::Engine::new();
    if let Some(_stats) = eng.go(&ckt, &cfg) {
        println!("\n*INFO* Done");
    } else {
        println!("\n*ERROR* Bad, bad bad... '{}'", &args[1]);
    }

}

