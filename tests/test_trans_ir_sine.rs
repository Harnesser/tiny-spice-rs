extern crate tiny_spice;

use tiny_spice::circuit::*;
use tiny_spice::engine;
use tiny_spice::analysis;

mod common;

#[test]
#[allow(non_snake_case)]
fn test_trans_ir_sine_10Hz() {

    let mut eng = engine::Engine::new();
    let mut cfg = analysis::Configuration::new();

    cfg.set_transient(2.0e-3, 10e-6, 0.0);
    cfg.set_wavefile("waves/trans_ir_sine_10Hz.dat");

    let ckt = build(10.0);
    let stats = eng.transient_analysis(&ckt, &cfg);
    println!("\n*INFO* Done");

    assert!(stats.end >= cfg.TSTOP);
}


#[test]
#[allow(non_snake_case)]
fn test_trans_ir_sine_1kHz() {

    let mut eng = engine::Engine::new();
    let mut cfg = analysis::Configuration::new();

    cfg.set_transient(2.0e-3, 10e-6, 0.0);
    cfg.set_wavefile("waves/trans_ir_sine_1kHz.dat");

    let ckt = build(1e3);
    let stats = eng.transient_analysis(&ckt, &cfg);
    println!("\n*INFO* Done");

    assert!(stats.end >= cfg.TSTOP);
}


fn build( freq: f64 ) -> Circuit {
    let mut ckt = Circuit::new();

    ckt.add_node("1");

    ckt.elements.push(
        Element::Isin(CurrentSourceSine{p: 0, n: 1, vo: 3.0, va: 1.0, freq: freq}),
    );
    ckt.elements.push(
        Element::R(Resistor{ident: "R1".to_string(), a: 1, b: 0, value: 10.0}),
    );

    ckt.build_node_id_lut();
    ckt
}

