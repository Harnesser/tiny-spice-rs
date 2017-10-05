extern crate tiny_spice;

use tiny_spice::circuit;
use tiny_spice::engine;

mod common;
use common::assert_nearly;

#[test]
fn test() {
    engine::banner();

    let mut eng = engine::Engine::new();
    let ckt = build();
    let (v,_) = eng.dc_operating_point(&ckt);
    println!("\n*INFO* Done");

    assert_nearly(v[1], 33.0);
    assert_nearly(v[2], 18.0);
    assert_nearly(v[3], 12.0);
}


// Example in `doc/Constructing_the_Voltage_Node_Matrix.odt`
// http://www3.imperial.ac.uk/pls/portallive/docs/1/7292571.PDF
//
// NGSPICE Result
// v(1) = -3.30000e+01
// v(2) = -1.80000e+01
// v(3) = -1.20000e+01
fn build() -> circuit::Circuit {
    let mut ckt = circuit::Circuit::new();
    ckt.elements.push(
        circuit::Element::I(circuit::CurrentSource{p: 0, n: 1, value: 3.0}),
    );
    ckt.elements.push(
        circuit::Element::R(circuit::Resistor{a: 1, b: 2, value: 5.0}),
    );
    ckt.elements.push(
        circuit::Element::R(circuit::Resistor{a: 2, b: 3, value: 5.0}),
    );
    ckt.elements.push(
        circuit::Element::R(circuit::Resistor{a: 2, b: 0, value: 10.0}),
    );
    ckt.elements.push(
        circuit::Element::R(circuit::Resistor{a: 3, b: 0, value: 10.0}),
    );
    ckt
}

