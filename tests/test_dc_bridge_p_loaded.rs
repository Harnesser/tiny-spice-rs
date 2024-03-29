extern crate tiny_spice;

use tiny_spice::circuit::*;
use tiny_spice::engine;
use tiny_spice::analysis;

mod common;
use crate::common::assert_nearly;

#[test]
fn test_dc_bridge_loaded_2v0() {

    let mut eng = engine::Engine::new();
    let mut cfg = analysis::Configuration::new();
    cfg.set_dc_operating_point();

    let ckt = build_v();
    let _ = eng.dc_operating_point(&ckt, &cfg);
    let v = eng.dc().unwrap();
    println!("\n*INFO* Done");

    assert_nearly(v[3], 9.4624);
    assert_nearly(v[4], 0.53759);
}

#[test]
fn test_dc_bridge_loaded_gnd() {

    let mut eng = engine::Engine::new();
    let mut cfg = analysis::Configuration::new();
    cfg.set_dc_operating_point();

    let ckt = build_vv();
    let _ = eng.dc_operating_point(&ckt, &cfg);
    let v = eng.dc().unwrap();

    println!("\n*INFO* Done");

    assert_nearly(v[3], 9.4624);
    assert_nearly(v[2], 0.53759);
}

#[allow(dead_code)]
fn build_v() -> Circuit {
    let mut ckt = Circuit::new();

    // bridge input voltage
    ckt.elements.push(Element::V(VoltageSource{p: 1, n: 2, value: 10.0, idx:0 }));
    ckt.elements.push(Element::V(VoltageSource{p: 2, n: 0, value: 0.0 , idx:1 }));

    // Diode bridge
    //  (1) is top
    //  (2) is bottom
    ckt.elements.push( Element::D(Diode::new("D1", 1, 3, 1e-9, 27.0)) );
    ckt.elements.push( Element::D(Diode::new("D2", 4, 1, 1e-9, 27.0)) );
    ckt.elements.push( Element::D(Diode::new("D3", 2, 3, 1e-9, 27.0)) );
    ckt.elements.push( Element::D(Diode::new("D4", 4, 2, 1e-9, 27.0)) );

    // load
    ckt.elements.push( Element::R(Resistor{ident: "R1".to_string(), a: 3, b: 4, value: 1000.0}) );

    ckt
}

#[allow(dead_code)]
fn build_vv() -> Circuit {
    let mut ckt = Circuit::new();

    // bridge input voltage
    ckt.elements.push(Element::V(VoltageSource{p: 1, n: 0, value: 10.0, idx:0}));

    // Diode bridge
    //  (1) is top
    //  (2) is bottom
    ckt.elements.push( Element::D(Diode::new("D1", 1, 3, 1e-9, 27.0)) );
    ckt.elements.push( Element::D(Diode::new("D2", 2, 1, 1e-9, 27.0)) );
    ckt.elements.push( Element::D(Diode::new("D3", 0, 3, 1e-9, 27.0)) );
    ckt.elements.push( Element::D(Diode::new("D4", 2, 0, 1e-9, 27.0)) );

    // load
    ckt.elements.push( Element::R(Resistor{ident: "r1".to_string(), a: 3, b: 2, value: 1000.0}) );

    ckt
}

