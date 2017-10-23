extern crate tiny_spice;

use tiny_spice::circuit::*;
use tiny_spice::engine;

#[test]
fn test_trans_ir_bridge_rc_1kHz() {

    let mut eng = engine::Engine::new();
    eng.TSTEP = 1.0e-6;
    let ckt = build(2.0, 1e3, 1e-9);
    let stats = eng.transient_analysis(&ckt, "waves/trans_ir_bridge_1kHz_rc_load.dat");
    println!("\n*INFO* Done");
    println!("{}", stats);
    assert!(stats.end >= eng.TSTOP);
}

#[allow(dead_code)]
fn build(amp: f32, freq: f32, isat: f32) -> Circuit {
    let mut ckt = Circuit::new();

    // bridge input voltage
    //ckt.elements.push(Element::V(VoltageSource{p: 1, n: 2, value: 10.0}));
    ckt.elements.push(
        Element::Isin(CurrentSourceSine{p: 0, n: 1, vo: 0.0, va: amp, freq: freq}),
    );
    ckt.elements.push(
        Element::R(Resistor{a: 1, b: 0, value: 10.0}),
    );


    ckt.elements.push(Element::V(VoltageSource{p: 2, n: 0, value: 0.0}));

    // Diode bridge
    //  (1) is top
    //  (2) is bottom
    ckt.elements.push( Element::D(Diode::new(1, 3, isat, 27.0)) );
    ckt.elements.push( Element::D(Diode::new(4, 1, isat, 27.0)) );
    ckt.elements.push( Element::D(Diode::new(2, 3, isat, 27.0)) );
    ckt.elements.push( Element::D(Diode::new(4, 2, isat, 27.0)) );

    // load
    ckt.elements.push( Element::R(Resistor{a: 3, b: 4, value: 1000.0}) );
    ckt.elements.push( Element::C(Capacitor{a: 3, b: 4, value: 1e-6}) );

    ckt
}

