/// Read in the opamp macromodel and test some values

use std::path::Path;

extern crate tiny_spice;

use tiny_spice::spice;
use tiny_spice::engine;
use tiny_spice::element::Element;

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

    let mut ckt = reader.get_expanded_circuit();
    let cfg = reader.configuration();

    // find the sinewave source and hack the offset to 5.0V to get
    // a non-zero dc value
    for el in &mut ckt.elements {
        match el {
            Element::Vsin(ref mut src) => {
                src.vo = 5.0;
            },
            _ => {},
        }
    }

    let _ = eng.dc_operating_point(&ckt, &cfg);
    let v = eng.dc().unwrap();

    assert_nearly(-50.0, v[3]);

}

