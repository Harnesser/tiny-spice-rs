extern crate tiny_spice;

use tiny_spice::circuit;
use tiny_spice::engine;


fn main() {
    engine::banner();

    let mut eng = engine::Engine::new();

    let ckt = build_002();
    eng.elaborate(&ckt);

    println!("*INFO* Done");
}


#[allow(dead_code)]
fn build_001() -> circuit::Circuit {
    let mut ckt = circuit::Circuit::new();
    ckt.elements.push(
        circuit::Element::I(circuit::CurrentSource{p: 1, n: 0, value: 3.0}),
    );
    ckt.elements.push(
        circuit::Element::R(circuit::Resistor{a: 1, b: 0, value: 10.0}),
    );
    ckt
}


#[allow(dead_code)]
// Example in `doc/Constructing_the_Voltage_Node_Matrix.odt`
// http://www3.imperial.ac.uk/pls/portallive/docs/1/7292571.PDF 
fn build_002() -> circuit::Circuit {
    let mut ckt = circuit::Circuit::new();
    ckt.elements.push(
        circuit::Element::I(circuit::CurrentSource{p: 1, n: 0, value: 3.0}),
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

