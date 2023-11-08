/// Simple DC test of VCCS and VCVS

use std::path::Path;

extern crate tiny_spice;

use tiny_spice::spice;
use tiny_spice::engine;
use tiny_spice::element::Element;

mod common;
use crate::common::assert_nearly;

#[test]
fn test_vc_vs_cs_basic() {

    // Initialise the SPICE Engine
    //  (do this first to get the banner at the top)
    let mut eng = engine::Engine::new();

    // See if the filename exists
    let spice_file = Path::new("./ngspice/vc_vs_cs_basic.spi");
    spice_file.try_exists().expect("Can't access spice file");

    let mut reader = spice::Reader::new();
    let errors_exist = reader.read(spice_file);
    if errors_exist {
        println!("*FATAL* Errors in SPICE Deck so not doing simulations");
        return;
    }

    let mut ckt = reader.get_expanded_circuit();
    let cfg = reader.configuration();

    // find the sinewave source and hack the offset to get a non-zero dc value
    for el in &mut ckt.elements {
        match el {
            Element::Isin(ref mut src) => {
                src.vo = 3.0;
            },
            _ => {},
        }
    }

    let stats = eng.dc_operating_point(&ckt, &cfg);
    let v = eng.dc().unwrap();

    assert_nearly(v[1], 3.0);
    assert_nearly(v[2], 6.0); // vccs
    assert_nearly(v[3], 9.0); // vcvs

    // linear solve should only take 2 steps to converge
    assert_eq!(stats.iterations, 2);

}

