extern crate tiny_spice;

use tiny_spice::circuit::*;
use tiny_spice::engine;
use tiny_spice::analysis;

mod common;
use crate::common::assert_nearly;

#[test]
fn test_ird_vv() {

    let mut eng = engine::Engine::new();
    let mut cfg = analysis::Configuration::new();

    cfg.set_dc_operating_point();

    let ckt = build();
    let _ = eng.dc_operating_point(&ckt, &cfg);
    let v = eng.dc().unwrap();
    println!("\n*INFO* Done");

    assert_nearly(v[1], 0.73217);
    assert_nearly(v[2], 0.73217); // 0V source
    assert_nearly(v[3], 0.73217); // 0V source
    assert_nearly(v[4] + v[5], 3.0); // branch current
}


fn build() -> Circuit {
    let mut ckt = Circuit::new();
    ckt.elements.push(
        Element::I(CurrentSource{p: 0, n: 1, value: 3.0}),
    );

    // R with series 0V source to measure branch current
    ckt.elements.push(
        Element::V(VoltageSource{p: 1, n: 2, value: 0.0}),
    );

    ckt.elements.push(
        Element::R(Resistor{a: 2, b: 0, value: 10.0}),
    );

    // D with series 0V source to measure branch current
    ckt.elements.push(
        Element::V(VoltageSource{p: 1, n: 3, value: 0.0}),
    );
    ckt.elements.push(
        Element::D(Diode::new(3, 0, 1e-9, 27.0)),
    );
    ckt
}

