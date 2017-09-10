extern crate tiny_spice;

use tiny_spice::circuit::*;
use tiny_spice::engine;

mod common;
use common::assert_nearly;

#[test]
fn test() {
    engine::banner();

    let mut eng = engine::Engine::new();
    let ckt = build();
    let v = eng.dc_operating_point(&ckt);
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
    ckt.elements.push( Element::D(Diode{p: 1, n: 3, i_sat: 1e-9, tdegc: 27.0}) );
    ckt.elements.push( Element::D(Diode{p: 4, n: 1, i_sat: 1e-9, tdegc: 27.0}) );
    ckt.elements.push( Element::D(Diode{p: 2, n: 3, i_sat: 1e-9, tdegc: 27.0}) );
    ckt.elements.push( Element::D(Diode{p: 4, n: 2, i_sat: 1e-9, tdegc: 27.0}) );

    // load
    // (none)

    ckt
}

