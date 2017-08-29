extern crate tiny_spice;

use tiny_spice::circuit;
use tiny_spice::engine;


fn main() {
    engine::banner();

    let mut eng = engine::Engine::new();

    let ckt = build_001();
    eng.elaborate(&ckt);

    println!("*INFO* Done");
}


fn build_001() -> circuit::Circuit {
    let mut ckt = circuit::Circuit::new();
    ckt.elements.push(
        circuit::Element::I(circuit::CurrentSource{p: 1, n: 0, value: 3.0}),
    );
    ckt.elements.push(
        circuit::Element::R(circuit::Resistor{a: 1, b: 0, value: 10.0}),
    );
    ckt
}
