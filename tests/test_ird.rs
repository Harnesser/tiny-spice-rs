extern crate tiny_spice;

use tiny_spice::circuit;
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

    assert_nearly(v[1], 0.79);
}


fn build() -> circuit::Circuit {
    let mut ckt = circuit::Circuit::new();
    ckt.elements.push(
        circuit::Element::I(circuit::CurrentSource{p: 0, n: 1, value: 3.0}),
    );
    ckt.elements.push(
        circuit::Element::R(circuit::Resistor{a: 1, b: 0, value: 10.0}),
    );
    ckt.elements.push(
        circuit::Element::D(circuit::Diode{p: 1, n: 0, i_sat: 1e-9, tdegc: 27.0}),
    );
    ckt
}

