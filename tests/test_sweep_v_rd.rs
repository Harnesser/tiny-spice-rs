extern crate tiny_spice;

use tiny_spice::circuit::*;
use tiny_spice::engine;
use tiny_spice::analysis;

/*
fn test_sweep_v_rd() {

    let mut eng = engine::Engine::new();
    let cfg = analysis::Configuration::new();

    //cfg.set_transient(2.0e-3, 10e-6, 0.0);
    //cfg.set_wavefile("waves/trans_ird_sine_1kHz_10us.dat");

    let ckt = build();
    let _v = eng.dc_sweep(&ckt, &cfg);
    println!("\n*INFO* Done");

    assert!(false);
}


fn build() -> Circuit {
    let mut ckt = Circuit::new();

    ckt.elements.push(Element::V(VoltageSource{p: 1, n: 0, value: 0.0}) );
    ckt.elements.push(Element::R(Resistor{a: 1, b: 2, value: 1e3}) );
    ckt.elements.push(Element::D(Diode::new(2, 0, 1e-9, 27.0)) );
    ckt
}
*/
