/// Read in the opamp macromodel and test some values

use std::path::Path;

extern crate tiny_spice;

use tiny_spice::spice;
use tiny_spice::engine;

mod common;
use crate::common::assert_nearly;

#[test]
fn test_opamp_basic() {

    // Initialise the SPICE Engine
    //  (do this first to get the banner at the top)
    let mut eng = engine::Engine::new();

    // See if the filename exists
    let spice_file = Path::new("./ngspice/opamp_basic.spi");
    spice_file.try_exists().expect("Can't access spice file");

    let mut reader = spice::Reader::new();
    let errors_exist = reader.read(spice_file);
    if errors_exist {
        println!("*FATAL* Errors in SPICE Deck so not doing simulations");
        return;
    }

    let ckt = reader.get_expanded_circuit();
    let cfg = reader.configuration();

    let _ = eng.dc_operating_point(&ckt, &cfg);
    let v = eng.dc().unwrap();

    assert_nearly(-50.0, v[3]);

}

