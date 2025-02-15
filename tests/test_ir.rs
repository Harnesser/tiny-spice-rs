extern crate tiny_spice;

use tiny_spice::circuit;
use tiny_spice::engine;
use tiny_spice::analysis;

mod common;
use crate::common::assert_nearly;

#[test]
fn test_ir() {

    let mut eng = engine::Engine::new();
    let mut cfg = analysis::Configuration::new();

    cfg.set_dc_operating_point();

    let ckt = build();
    let _ = eng.dc_operating_point(&ckt, &cfg);
    let v = eng.dc().unwrap();
    println!("\n*INFO* Done");

    assert_nearly(v[1], 30.0);
}


fn build() -> circuit::Circuit {
    let mut ckt = circuit::Circuit::new();
    ckt.elements.push(
        circuit::Element::I(circuit::CurrentSource{p: 0, n: 1, value: 3.0}),
    );
    ckt.elements.push(
        circuit::Element::R(circuit::Resistor{ident: "R100".to_string(), a: 1, b: 0, value: 10.0}),
    );
    ckt
}

