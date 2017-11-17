extern crate tiny_spice;

use tiny_spice::circuit::*;
use tiny_spice::engine;
use tiny_spice::analysis;

mod common;

#[test]
#[allow(non_snake_case)]
fn test_irrc_trans_hpf_1kHz() {

    let mut eng = engine::Engine::new();
    let mut cfg = analysis::Configuration::new();

    cfg.set_transient(2.0e-3, 1e-6, 0.0);
    cfg.set_wavefile("waves/trans_irrc_hpf_1kHz.dat");

    let ckt = build(1.0e3);
    let stats = eng.transient_analysis(&ckt, &cfg);
    println!("\n*INFO* Done");

    assert!(stats.end >= cfg.TSTOP);
}

#[test]
#[allow(non_snake_case)]
fn test_irrc_trans_hpf_2kHz() {

    let mut eng = engine::Engine::new();
    let mut cfg = analysis::Configuration::new();

    cfg.set_transient(2.0e-3, 1e-6, 0.0);
    cfg.set_wavefile("waves/trans_irrc_hpf_2kHz.dat");

    let ckt = build(2.0e3);
    let stats = eng.transient_analysis(&ckt, &cfg);
    println!("\n*INFO* Done");

    assert!(stats.end >= cfg.TSTOP);
}

#[test]
#[allow(non_snake_case)]
fn test_irrc_trans_hpf_5kHz() {

    let mut eng = engine::Engine::new();
    let mut cfg = analysis::Configuration::new();

    cfg.set_transient(2.0e-3, 1e-6, 0.0);
    cfg.set_wavefile("waves/trans_irrc_hpf_5kHz.dat");

    let ckt = build(5.0e3);
    let stats = eng.transient_analysis(&ckt, &cfg);
    println!("\n*INFO* Done");

    assert!(stats.end >= cfg.TSTOP);
}

#[test]
#[allow(non_snake_case)]
fn test_irrc_trans_hpf_10kHz() {

    let mut eng = engine::Engine::new();
    let mut cfg = analysis::Configuration::new();

    cfg.set_transient(2.0e-3, 1e-6, 0.0);
    cfg.set_wavefile("waves/trans_irrc_hpf_10kHz.dat");

    let ckt = build(10.0e3);
    let stats = eng.transient_analysis(&ckt, &cfg);
    println!("\n*INFO* Done");

    assert!(stats.end >= cfg.TSTOP);
}


fn build( freq: f64 ) -> Circuit {
    let mut ckt = Circuit::new();

    // 10V Voltage Source
    ckt.elements.push(
        Element::Isin(CurrentSourceSine{p: 0, n: 1, vo: 0.0, va: 10.0, freq: freq}),
    );
    ckt.elements.push(
        Element::R(Resistor{a: 1, b: 0, value: 1.0}),
    );

    // High-pass filter - 5kHz cut-off
    ckt.elements.push(
        Element::C(Capacitor{a: 1, b: 2, value: 0.032e-6}),
    );
    ckt.elements.push(
        Element::R(Resistor{a: 2, b: 0, value: 1.0e3}),
    );
    ckt
}

