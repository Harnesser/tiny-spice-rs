extern crate tiny_spice;

use tiny_spice::circuit::*;
use tiny_spice::engine;
use tiny_spice::analysis;

mod common;

#[test]
#[allow(non_snake_case)]
fn test_trans_irrd_sine_1kHz_10us() {

    let mut eng = engine::Engine::new();
    let mut cfg = analysis::Configuration::new();

    cfg.set_transient(2.0e-3, 10e-6, 0.0);
    cfg.set_wavefile("waves/trans_irrd_sine_1kHz_10us.dat");

    let ckt = build(1e3);
    let stats = eng.transient_analysis(&ckt, &cfg);
    println!("\n*INFO* Done");

    assert!(stats.end >= cfg.TSTOP);
}


#[test]
#[allow(non_snake_case)]
fn test_trans_irrd_sine_1kHz_1us() {

    let mut eng = engine::Engine::new();
    let mut cfg = analysis::Configuration::new();

    cfg.set_transient(2.0e-3, 1e-6, 0.0);
    cfg.set_wavefile("waves/trans_irdd_sine_1kHz_1us.dat");

    let ckt = build(1e3);
    let stats = eng.transient_analysis(&ckt, &cfg);
    println!("\n*INFO* Done");

    assert!(stats.end >= cfg.TSTOP);
}


#[test]
#[ignore]
fn test_trans_irrd_sine_loop() {

    let timesteps = [10e-6, 5e-6, 2e-6, 1e-6];
    let freqs = [3.0e3, 2.5e3, 2.0e3, 1.0e3, 0.5e3, 0.4e3, 0.3e3, 0.2e3, 0.1e3, 0.05e3];

    let mut i = 0;
    for timestep in timesteps.iter() {
        for freq in freqs.iter() {

            let mut eng = engine::Engine::new();
            let mut cfg = analysis::Configuration::new();
            cfg.set_transient(2.0e-3, *timestep, 0.0);

            let ckt = build(*freq);

            let filename = format!("waves/test_trans_irrd_sine_loop/{:03}.dat", i);
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
    ckt.elements.push(
        Element::Isin(CurrentSourceSine{p: 0, n: 1, vo: 0.0, va: 0.3, freq: freq}),
    );
    ckt.elements.push(
        Element::R(Resistor{a: 1, b: 0, value: 10.0}),
    );
    ckt.elements.push(
        Element::D(Diode::new(1, 2, 1e-9, 27.0)),
    );
    ckt.elements.push(
        Element::R(Resistor{a: 2, b: 0, value: 1e3}),
    );
    ckt
}

