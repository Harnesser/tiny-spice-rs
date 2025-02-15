extern crate tiny_spice;

use tiny_spice::circuit::*;
use tiny_spice::engine;
use tiny_spice::analysis;

mod common;

#[test]
#[allow(non_snake_case)]
fn test_irrc_trans_1kHz() {

    let mut eng = engine::Engine::new();
    let mut cfg = analysis::Configuration::new();

    cfg.set_transient(2.0e-3, 1e-6, 0.0);
    cfg.set_wavefile("waves/trans_irrc_1kHz.dat");

    let ckt = build(1.0e3);
    let stats = eng.transient_analysis(&ckt, &cfg);
    println!("\n*INFO* Done");

    assert!(stats.end >= cfg.TSTOP);
}

#[test]
#[allow(non_snake_case)]
fn test_irrc_trans_2kHz() {

    let mut eng = engine::Engine::new();
    let mut cfg = analysis::Configuration::new();

    cfg.set_transient(2.0e-3, 1e-6, 0.0);
    cfg.set_wavefile("waves/trans_irrc_2kHz.dat");

    let ckt = build(2.0e3);
    let stats = eng.transient_analysis(&ckt, &cfg);
    println!("\n*INFO* Done");

    assert!(stats.end >= cfg.TSTOP);
}

#[test]
#[allow(non_snake_case)]
fn test_irrc_trans_5kHz() {

    let mut eng = engine::Engine::new();
    let mut cfg = analysis::Configuration::new();

    cfg.set_transient(2.0e-3, 1e-6, 0.0);
    cfg.set_wavefile("waves/trans_irrc_5kHz.dat");

    let ckt = build(5.0e3);
    let stats = eng.transient_analysis(&ckt, &cfg);
    println!("\n*INFO* Done");

    assert!(stats.end >= cfg.TSTOP);
}

#[test]
#[allow(non_snake_case)]
fn test_irrc_trans_10kHz() {

    let mut eng = engine::Engine::new();
    let mut cfg = analysis::Configuration::new();

    cfg.set_transient(2.0e-3, 1e-6, 0.0);
    cfg.set_wavefile("waves/trans_irrc_10kHz.dat");

    let ckt = build(10.0e3);
    let stats = eng.transient_analysis(&ckt, &cfg);
    println!("\n*INFO* Done");

    assert!(stats.end >= cfg.TSTOP);
}


#[test]
#[ignore]
fn test_irrc_trans_lpf_loop() {

    let timesteps = [10e-6, 5e-6, 2e-6, 1e-6];
    let freqs = [3.0e3, 2.5e3, 2.0e3, 1.0e3, 0.5e3, 0.4e3, 0.3e3, 0.2e3, 0.1e3, 0.05e3];

    let mut i = 0;
    for timestep in timesteps.iter() {
        for freq in freqs.iter() {

            let mut eng = engine::Engine::new();
            let mut cfg = analysis::Configuration::new();
            cfg.set_transient(2.0e-3, *timestep, 0.0);

            let ckt = build(*freq);

            let filename = format!("waves/test_trans_irrc_lpf_loop/{:03}.dat", i);
            cfg.set_wavefile(&filename);
            let stats = eng.transient_analysis(&ckt, &cfg);
            println!("{}", stats);
            if stats.end >= cfg.TSTOP {
                println!("LOOPRESULT {} {} GOOD\n\n", timestep, freq);
            } else {
                println!("LOOPRESULT {} {} BAD\n\n", timestep, freq);
            }

            i += 1;
        }
    }
}


fn build( freq: f64 ) -> Circuit {
    let mut ckt = Circuit::new();

    ckt.add_node("1");
    ckt.add_node("2");

    // 10V Voltage Source
    ckt.elements.push(
        Element::Isin(CurrentSourceSine{p: 0, n: 1, vo: 0.0, va: 10.0, freq: freq}),
    );
    ckt.elements.push(
        Element::R(Resistor{ident: "r1".to_string(), a: 1, b: 0, value: 1.0}),
    );

    // Lowpass filter - 5kHz cut-off
    ckt.elements.push(
        Element::R(Resistor{ident: "r2".to_string(), a: 1, b: 2, value: 1.0e3}),
    );
    ckt.elements.push(
        Element::C(Capacitor{ident: "c1".to_string(), a: 2, b: 0, value: 0.032e-6}),
    );

    ckt.build_node_id_lut();
    ckt
}

