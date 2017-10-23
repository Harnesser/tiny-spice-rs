extern crate tiny_spice;

use tiny_spice::circuit::*;
use tiny_spice::engine;

mod common;
use common::assert_nearly;

#[test]
fn test() {

    let mut eng = engine::Engine::new();
    let ckt = build();
    let (v,_) = eng.dc_operating_point(&ckt);
    println!("\n*INFO* Done");

    assert_nearly(v[3], 9.809084);
    assert_nearly(v[4], 0.190917);
}


fn build() -> Circuit {
    let mut ckt = Circuit::new();

    // bridge input voltage
    ckt.elements.push(Element::V(VoltageSource{p: 1, n: 2, value: 10.0}));
    ckt.elements.push(Element::V(VoltageSource{p: 2, n: 0, value: 0.0}));

    // Diode bridge
    //  (1) is top
    //  (2) is bottom
    ckt.elements.push( Element::D(Diode::new(1, 3, 1e-9, 27.0)) );
    ckt.elements.push( Element::D(Diode::new(4, 1, 1e-9, 27.0)) );
    ckt.elements.push( Element::D(Diode::new(2, 3, 1e-9, 27.0)) );
    ckt.elements.push( Element::D(Diode::new(4, 2, 1e-9, 27.0)) );

    // load
    // (none)

    ckt
}

