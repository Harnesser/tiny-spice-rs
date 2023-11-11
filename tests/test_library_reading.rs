/// Check that only the `.lib` regions of files are considered
/// for the toplevel circuit when called as a library. The toplevel 
/// file in this test calls two other SPICE files that both have
/// subckts in `.lib` sections and other supporting circuitry outside
/// the `.lib` definitions for local testing.
///
/// Demonstrates:
/// 1. that the library files can be found relative to the toplevel
///    SPICE file
/// 2. that the only elements included in the toplevel circuit are
///    things within the named `.lib` and `.endl` sections.
///
/// Pass/Fail:
/// * no panics or errors
/// * node count ok
/// * primitive element count ok

use std::path::Path;

extern crate tiny_spice;

use tiny_spice::spice;
use tiny_spice::engine;

mod common;
use crate::common::assert_nearly;

#[test]
fn test_library_reading() {

    // Initialise the SPICE Engine
    // (do this first to get the banner at the top)
    let mut eng = engine::Engine::new();

    let spice_file = Path::new("./ngspice/drum-machine/libtest.spi");

    let mut reader = spice::Reader::new();
    let errors_exist = reader.read(spice_file);
    if errors_exist {
        panic!("*FATAL* Errors in SPICE Deck so not doing simulations");
        return;
    }

    let ckt = reader.get_expanded_circuit();
    let cfg = reader.configuration();

    let _ = eng.dc_operating_point(&ckt, &cfg);
    let v = eng.dc().unwrap();

    assert_eq!(ckt.node_id_lut.len(), 8);
    assert_eq!(ckt.elements.len(), 13);

}

