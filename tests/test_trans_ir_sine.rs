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
    let v = eng.transient_analysis(&ckt);
    println!("\n*INFO* Done");

    assert!(false);
}


fn build() -> Circuit {
    let mut ckt = Circuit::new();
    ckt.elements.push(
        Element::Isin(CurrentSourceSine{p: 0, n: 1, vo: 3.0, va: 1.0, freq: 10e3}),
    );
    ckt.elements.push(
        Element::R(Resistor{a: 1, b: 0, value: 10.0}),
    );
    ckt
}

