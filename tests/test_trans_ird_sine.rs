extern crate tiny_spice;

use tiny_spice::circuit::*;
use tiny_spice::engine;


#[test]
#[allow(non_snake_case)]
fn test_trans_ir_sine_1kHz_10us() {

    let mut eng = engine::Engine::new();
    eng.TSTEP = 10e-6;
    let ckt = build(2.0, 1e3, 1e-9);
    let stats = eng.transient_analysis(&ckt, "waves/trans_ird_sine_1kHz_10us.dat");
    println!("\n*INFO* Done");
    assert!(stats.end >= eng.TSTOP);
}


#[test]
#[allow(non_snake_case)]
fn test_trans_ir_sine_1kHz_1us() {

    let mut eng = engine::Engine::new();
    eng.TSTEP = 1e-6;
    let ckt = build(2.0, 1e3, 1e-9);
    let stats = eng.transient_analysis(&ckt, "waves/trans_ird_sine_1kHz_1us.dat");
    println!("\n*INFO* Done");
    assert!(stats.end >= eng.TSTOP);
}



fn build( amp: f32, freq: f32, isat: f32 ) -> Circuit {
    let i_offset = 0.0;
    let mut ckt = Circuit::new();
    ckt.elements.push(
        Element::Isin(CurrentSourceSine{p: 0, n: 1, vo: i_offset, va: amp, freq: freq}),
    );
    ckt.elements.push(
        Element::R(Resistor{a: 1, b: 0, value: 10.0}),
    );
    ckt.elements.push(
        Element::D(Diode{p: 1, n: 0, i_sat: isat, tdegc: 27.0}),
    );
    ckt
}

