extern crate tiny_spice;

use tiny_spice::circuit::*;
use tiny_spice::engine;
use tiny_spice::analysis;

mod common;
use common::assert_nearly;

#[test]
fn test_irrc_dc() {

    let mut eng = engine::Engine::new();
    let mut cfg = analysis::Configuration::new();

    cfg.set_dc_operating_point();

    let ckt = build();
    let _ = eng.dc_operating_point(&ckt, &cfg);
    let v = eng.dc().unwrap();
    println!("\n*INFO* Done");

    assert_nearly(v[2], 9.999999);
}


fn build() -> Circuit {
    let mut ckt = Circuit::new();

    // 10V Voltage Source
    ckt.elements.push(
        Element::I(CurrentSource{p: 0, n: 1, value: 1.0}),
    );
    ckt.elements.push(
        Element::R(Resistor{a: 1, b: 0, value: 10.0}),
    );

    // Lowpass filter
    ckt.elements.push(
        Element::R(Resistor{a: 1, b: 2, value: 1.0e-3}),
    );
    ckt.elements.push(
        Element::C(Capacitor{a: 2, b: 0, value: 0.01e-6}),
    );
    ckt
}

