extern crate tiny_spice;

use tiny_spice::circuit::*;
use tiny_spice::engine;

mod common;
use common::assert_nearly;

#[test]
fn test_trans_ir_bridge_1kHz() {
    engine::banner();

    let mut eng = engine::Engine::new();
    let ckt = build(1e3);
    let v = eng.transient_analysis(&ckt, "waves/trans_ir_bridge_1kHz.dat");
    println!("\n*INFO* Done");

    assert!(false);
}

#[allow(dead_code)]
fn build(freq: f32) -> Circuit {
    let mut ckt = Circuit::new();

    // bridge input voltage
    //ckt.elements.push(Element::V(VoltageSource{p: 1, n: 2, value: 10.0}));
    ckt.elements.push(
        Element::Isin(CurrentSourceSine{p: 0, n: 1, vo: 0.0, va: 2.0, freq: freq}),
    );
    ckt.elements.push(
        Element::R(Resistor{a: 1, b: 0, value: 10.0}),
    );


    ckt.elements.push(Element::V(VoltageSource{p: 2, n: 0, value: 0.0}));

    // Diode bridge
    //  (1) is top
    //  (2) is bottom
    ckt.elements.push( Element::D(Diode{p: 1, n: 3, i_sat: 1e-9, tdegc: 27.0}) );
    ckt.elements.push( Element::D(Diode{p: 4, n: 1, i_sat: 1e-9, tdegc: 27.0}) );
    ckt.elements.push( Element::D(Diode{p: 2, n: 3, i_sat: 1e-9, tdegc: 27.0}) );
    ckt.elements.push( Element::D(Diode{p: 4, n: 2, i_sat: 1e-9, tdegc: 27.0}) );

    // load
    ckt.elements.push( Element::R(Resistor{a: 3, b: 4, value: 1000.0}) );

    ckt
}

