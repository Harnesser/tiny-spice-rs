extern crate tiny_spice;

use tiny_spice::circuit;
use tiny_spice::engine;


fn main() {

    let mut eng = engine::Engine::new();

    let ckt_002 = build_002();
    eng.dc_operating_point(&ckt_002);

    let ckt_004 = build_004();
    eng.dc_operating_point(&ckt_004);

    println!("\n*INFO* Done");
}


#[allow(dead_code)]
fn build_001() -> circuit::Circuit {
    let mut ckt = circuit::Circuit::new();
    ckt.elements.push(
        circuit::Element::I(circuit::CurrentSource{p: 0, n: 1, value: 3.0}),
    );
    ckt.elements.push(
        circuit::Element::R(circuit::Resistor{a: 1, b: 0, value: 10.0}),
    );
    ckt
}


#[allow(dead_code)]
// Example in `doc/Constructing_the_Voltage_Node_Matrix.odt`
// http://www3.imperial.ac.uk/pls/portallive/docs/1/7292571.PDF
//
// NGSPICE Result
// v(1) = -3.30000e+01
// v(2) = -1.80000e+01
// v(3) = -1.20000e+01
fn build_002() -> circuit::Circuit {
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




// build_003 - I, R & Diode




// from https://www.swarthmore.edu/NatSci/echeeve1/Ref/mna/MNA2.html
// (Example 2)
//
// NGSPICE result
// a = -8.00000e+00
// b = 2.400000e+01
// c = 2.000000e+01
// v1#branch = -4.00000e+00
// v2#branch = 1.000000e+00
fn build_004() -> circuit::Circuit {
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


