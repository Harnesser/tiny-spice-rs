extern crate tiny_spice;

use tiny_spice::circuit::*;
use tiny_spice::engine;

mod common;
use common::assert_nearly;

#[test]
fn test_irrc_trans_1kHz() {
    engine::banner();

    let mut eng = engine::Engine::new();
    eng.TSTEP = 1e-6;
    let ckt = build(1.0e3);
    let v = eng.transient_analysis(&ckt, "waves/trans_irrc_1kHz.dat");
    println!("\n*INFO* Done");

    assert!(false);
}

#[test]
fn test_irrc_trans_2kHz() {
    engine::banner();

    let mut eng = engine::Engine::new();
    eng.TSTEP = 1e-6;
    let ckt = build(2.0e3);
    let v = eng.transient_analysis(&ckt, "waves/trans_irrc_2kHz.dat");
    println!("\n*INFO* Done");

    assert!(false);
}

#[test]
fn test_irrc_trans_5kHz() {
    engine::banner();

    let mut eng = engine::Engine::new();
    eng.TSTEP = 1e-6;
    let ckt = build(5.0e3);
    let v = eng.transient_analysis(&ckt, "waves/trans_irrc_5kHz.dat");
    println!("\n*INFO* Done");

    assert!(false);
}

#[test]
fn test_irrc_trans_10kHz() {
    engine::banner();

    let mut eng = engine::Engine::new();
    eng.TSTEP = 1e-6;
    let ckt = build(10.0e3);
    let v = eng.transient_analysis(&ckt, "waves/trans_irrc_10kHz.dat");
    println!("\n*INFO* Done");

    assert!(false);
}


fn build( freq: f32 ) -> Circuit {
    let mut ckt = Circuit::new();

    // 10V Voltage Source
    ckt.elements.push(
        Element::Isin(CurrentSourceSine{p: 0, n: 1, vo: 0.0, va: 10.0, freq: freq}),
    );
    ckt.elements.push(
        Element::R(Resistor{a: 1, b: 0, value: 1.0}),
    );

    // Lowpass filter - 5kHz cut-off
    ckt.elements.push(
        Element::R(Resistor{a: 1, b: 2, value: 1.0e3}),
    );
    ckt.elements.push(
        Element::C(Capacitor{a: 2, b: 0, value: 0.032e-6}),
    );
    ckt
}

