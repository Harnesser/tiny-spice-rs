extern crate tiny_spice;

use tiny_spice::circuit;
use tiny_spice::engine;

mod common;
use common::assert_nearly;

#[test]
fn test() {

    let mut eng = engine::Engine::new();
    let ckt = build();
    let (v,_) = eng.dc_operating_point(&ckt);
    println!("\n*INFO* Done");

    assert_nearly(v[1], -8.0);
    assert_nearly(v[2], 24.0);
    assert_nearly(v[3], 20.0);
    assert_nearly(v[4], -4.0);
    assert_nearly(v[5],  1.0);
}


// from https://www.swarthmore.edu/NatSci/echeeve1/Ref/mna/MNA2.html
// (Example 2)
//
// NGSPICE result
// a = -8.00000e+00
// b = 2.400000e+01
// c = 2.000000e+01
// v1#branch = -4.00000e+00
// v2#branch = 1.000000e+00
fn build() -> circuit::Circuit {
    let mut ckt = circuit::Circuit::new();
    ckt.elements.push(
        circuit::Element::R(circuit::Resistor{a: 0, b: 1, value: 2.0}),
    );
    ckt.elements.push(
        circuit::Element::V(circuit::VoltageSource{p: 2, n: 1, value: 32.0}),
    );
    ckt.elements.push(
        circuit::Element::R(circuit::Resistor{a: 2, b: 3, value: 4.0}),
    );
    ckt.elements.push(
        circuit::Element::R(circuit::Resistor{a: 2, b: 0, value: 8.0}),
    );
    ckt.elements.push(
        circuit::Element::V(circuit::VoltageSource{p: 3, n: 0, value: 20.0}),
    );
    ckt
}


