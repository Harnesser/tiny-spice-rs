extern crate tiny_spice;

use std::env;
use std::path::Path;

use tiny_spice::spice;
use tiny_spice::engine;


/// Read a spice file, and execute it
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please supply a SPICE filename");
    }

    // Initialise the SPICE Engine
    //  (do this first to get the banner at the top)
    let mut eng = engine::Engine::new();

    // See if the filename exists
    let spice_file = Path::new(&args[1]);
    spice_file.try_exists().expect("Can't access spice file");

    let mut reader = spice::Reader::new();
    let errors_exist = reader.read(spice_file);
    if errors_exist {
        println!("*FATAL* Errors in SPICE Deck so not doing simulations");
        return;
    }

    let ckt = reader.circuit();
    ckt.show();
    let cfg = reader.configuration();

    // tmp analysis
    if let Some(_stats) = eng.go(ckt, cfg) {
        println!("\n*INFO* Done");
    } else {
        println!("\n*ERROR* Bad, bad bad... '{}'", &args[1]);
    }

}

