extern crate tiny_spice;

use tiny_spice::circuit::*;
use tiny_spice::engine;
use tiny_spice::analysis;

mod common;
use crate::common::assert_nearly;

#[test]
fn test_v_d_vs_d() {

    let mut eng = engine::Engine::new();
    let mut cfg = analysis::Configuration::new();
    cfg.set_dc_operating_point();

    let ckt = build();
    let _ = eng.dc_operating_point(&ckt, &cfg);
    let v = eng.dc().unwrap();

    println!("\n*INFO* Done");
    assert_nearly(v[1], 5.0);
}


// Voltage source with a set of diodes
// (1) V --> D1 (fwd) (2) --> D2(rev) (0)
fn build() -> Circuit {
    let mut ckt = Circuit::new();
    ckt.elements.push( Element::V(VoltageSource{p: 1, n: 0, value: 5.0, idx:0}) );
    // 'forward' biased
    ckt.elements.push( Element::D(Diode::new(1, 2, 1e-9, 27.0)) );
    // reversed bias
    ckt.elements.push( Element::D(Diode::new(0, 2, 1e-9, 27.0)) );
    ckt
}

