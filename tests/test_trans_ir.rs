extern crate tiny_spice;

use tiny_spice::circuit;
use tiny_spice::engine;

mod common;

#[test]
fn trans_ir() {

    let mut eng = engine::Engine::new();
    let ckt = build();
    let stats = eng.transient_analysis(&ckt, "waves/trans_ir.dat");
    println!("\n*INFO* Done");
    
    assert!(stats.end >= eng.TSTOP);
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

