extern crate tiny_spice;

use tiny_spice::circuit::*;
use tiny_spice::engine;

#[test]
fn test_sweep_v_rd() {
    engine::banner();

    let mut eng = engine::Engine::new();
    let ckt = build();
    let v = eng.dc_sweep(&ckt, "waves/sweep_v_rd.dat");
    println!("\n*INFO* Done");

    assert!(false);
}


fn build() -> Circuit {
    let mut ckt = Circuit::new();

    ckt.elements.push(Element::V(VoltageSource{p: 1, n: 0, value: 0.0}) );
    ckt.elements.push(Element::R(Resistor{a: 1, b: 2, value: 1e3}) );
    ckt.elements.push(Element::D(Diode{p: 2, n: 0, i_sat: 1e-9, tdegc: 27.0}) );
    ckt
}

