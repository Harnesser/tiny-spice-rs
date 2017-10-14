extern crate tiny_spice;

use tiny_spice::circuit;
use tiny_spice::engine;

mod common;
use common::assert_nearly;

#[test]
fn test() {

    let mut eng = engine::Engine::new();
    let ckt = build();
    let v = eng.transient_analysis(&ckt, "waves/trans_ir.dat");
    println!("\n*INFO* Done");

    assert!(false);
}


fn build() -> circuit::Circuit {
    let mut ckt = circuit::Circuit::new();
    ckt.elements.push(
        circuit::Element::I(circuit::CurrentSource{p: 0, n: 1, value: 3.0}),
    );
    ckt.elements.push(
        circuit::Element::R(circuit::Resistor{a: 1, b: 0, value: 10.0}),
    );
    ckt
}

