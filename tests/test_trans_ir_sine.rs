extern crate tiny_spice;

use tiny_spice::circuit::*;
use tiny_spice::engine;

mod common;

#[test]
#[allow(non_snake_case)]
fn test_trans_ir_sine_10Hz() {

    let mut eng = engine::Engine::new();
    let ckt = build(10.0);
    let stats = eng.transient_analysis(&ckt, "waves/trans_ir_sine_10Hz.dat");
    println!("\n*INFO* Done");

    assert!(stats.end >= eng.TSTOP);
}


#[test]
#[allow(non_snake_case)]
fn test_trans_ir_sine_1kHz() {

    let mut eng = engine::Engine::new();
    let ckt = build(1e3);
    let stats = eng.transient_analysis(&ckt, "waves/trans_ir_sine_1kHz.dat");
    println!("\n*INFO* Done");

    assert!(stats.end >= eng.TSTOP);
}


fn build( freq: f64 ) -> Circuit {
    let mut ckt = Circuit::new();
    ckt.elements.push(
        Element::Isin(CurrentSourceSine{p: 0, n: 1, vo: 3.0, va: 1.0, freq: freq}),
    );
    ckt.elements.push(
        Element::R(Resistor{a: 1, b: 0, value: 10.0}),
    );
    ckt
}

