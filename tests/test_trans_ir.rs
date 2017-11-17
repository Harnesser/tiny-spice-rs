extern crate tiny_spice;

use tiny_spice::circuit;
use tiny_spice::engine;
use tiny_spice::analysis;

mod common;

#[test]
fn trans_ir() {

    let mut eng = engine::Engine::new();
    let mut cfg = analysis::Configuration::new();

    cfg.set_transient(2.0e-3, 10e-6, 0.0);
    cfg.set_wavefile("waves/trans_ir.dat");

    let ckt = build();
    let stats = eng.transient_analysis(&ckt, &cfg);
    println!("\n*INFO* Done");
    
    assert!(stats.end >= cfg.TSTOP);
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

