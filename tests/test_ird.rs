extern crate tiny_spice;

use tiny_spice::circuit;
use tiny_spice::engine;
use tiny_spice::analysis;

mod common;
use crate::common::assert_nearly;

#[test]
fn test_dc_ird() {

    let mut eng = engine::Engine::new();
    let mut cfg = analysis::Configuration::new();
    cfg.set_dc_operating_point();

    let ckt = build(1e-9);
    let _ = eng.dc_operating_point(&ckt, &cfg);
    let v = eng.dc().unwrap();

    println!("\n*INFO* Done");
    assert_nearly(v[1], 0.73217);
}


#[test]
#[allow(non_snake_case)]
fn test_dc_ird_isat_1pA() {

    let mut eng = engine::Engine::new();
    let mut cfg = analysis::Configuration::new();
    cfg.set_dc_operating_point();

    let ckt = build(1e-12);
    let _ = eng.dc_operating_point(&ckt, &cfg);
    let v = eng.dc().unwrap();

    println!("\n*INFO* Done");
    assert_nearly(v[1], 0.73217);
}


fn build(isat: f64) -> circuit::Circuit {
    let mut ckt = circuit::Circuit::new();
    ckt.elements.push(
        circuit::Element::I(circuit::CurrentSource{p: 0, n: 1, value: 3.0}),
    );
    ckt.elements.push(
        circuit::Element::R(circuit::Resistor{a: 1, b: 0, value: 10.0}),
    );
    ckt.elements.push(
        circuit::Element::D(circuit::Diode::new(1, 0, isat, 27.0)),
    );
    ckt
}

