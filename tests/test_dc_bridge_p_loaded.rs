extern crate tiny_spice;

use tiny_spice::circuit::*;
use tiny_spice::engine;

mod common;
use common::assert_nearly;

#[test]
fn test_v() {
    engine::banner();

    let mut eng = engine::Engine::new();
    let ckt = build_v();
    let (v, _) = eng.dc_operating_point(&ckt);
    println!("\n*INFO* Done");

    assert_nearly(v[3], 9.4624);
    assert_nearly(v[4], 0.53759);
}

#[test]
fn test_vv() {
    engine::banner();

    let mut eng = engine::Engine::new();
    let ckt = build_vv();
    let (v, _) = eng.dc_operating_point(&ckt);
    println!("\n*INFO* Done");

    assert_nearly(v[3], 9.4624);
    assert_nearly(v[2], 0.53759);
}

#[allow(dead_code)]
fn build_v() -> Circuit {
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
    ckt.elements.push( Element::R(Resistor{a: 3, b: 4, value: 1000.0}) );

    ckt
}

#[allow(dead_code)]
fn build_vv() -> Circuit {
    let mut ckt = Circuit::new();

    // bridge input voltage
    ckt.elements.push(Element::V(VoltageSource{p: 1, n: 0, value: 10.0}));

    // Diode bridge
    //  (1) is top
    //  (2) is bottom
    ckt.elements.push( Element::D(Diode{p: 1, n: 3, i_sat: 1e-9, tdegc: 27.0}) );
    ckt.elements.push( Element::D(Diode{p: 2, n: 1, i_sat: 1e-9, tdegc: 27.0}) );
    ckt.elements.push( Element::D(Diode{p: 0, n: 3, i_sat: 1e-9, tdegc: 27.0}) );
    ckt.elements.push( Element::D(Diode{p: 2, n: 0, i_sat: 1e-9, tdegc: 27.0}) );

    // load
    ckt.elements.push( Element::R(Resistor{a: 3, b: 2, value: 1000.0}) );

    ckt
}

