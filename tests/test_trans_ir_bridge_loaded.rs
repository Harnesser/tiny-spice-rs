extern crate tiny_spice;

use tiny_spice::circuit::*;
use tiny_spice::engine;

#[test]
#[allow(non_snake_case)]
fn test_trans_ir_bridge_1kHz_10us() {

    let mut eng = engine::Engine::new();
    eng.TSTEP = 10e-6;
    let ckt = build(2.0, 1.0e3, 1e-9);
    let stats = eng.transient_analysis(&ckt, "waves/trans_ir_bridge_1kHz_10us.dat");
    println!("\n*INFO* Done");
    println!("{}", stats);
    assert!(stats.end >= eng.TSTOP);
}


#[test]
//#[ignore]
#[allow(non_snake_case)]
fn test_trans_ir_bridge_1kHz_1us() {

    let mut eng = engine::Engine::new();
    eng.TSTEP = 1.0e-6;
    let ckt = build(2.0, 1.0e3, 1e-9);
    let stats = eng.transient_analysis(&ckt, "waves/trans_ir_bridge_1kHz_1us.dat");
    println!("\n*INFO* Done");
    println!("{}", stats);
    assert!(stats.end >= eng.TSTOP);
}

#[test]
#[ignore]
#[allow(non_snake_case)]
fn test_trans_ir_bridge_loaded_loop() {


    let timesteps = [10e-6, 5e-6, 2e-6, 1e-6];
    let amps = [-2.0, -1.0, -0.5, 0.5, 1.0, 2.0];
    let freqs = [3.0e3, 2.5e3, 2.0e3, 1.0e3, 0.5e3, 0.4e3, 0.3e3, 0.2e3, 0.1e3, 0.05e3];
    let isats = [1e-9, 1e-12, 1e-13];

/*
    let timesteps = [10e-6, 1e-6];
    let amps = [-2.0, 2.0];
    let freqs = [10.0e3, 0.1e3];
    let isats = [1e-9, 1e-13];
*/
    let mut i = 0;
    let mut fails = 0;
    for timestep in timesteps.iter() {
        for freq in freqs.iter() {
            for amp in amps.iter() {
                for isat in isats.iter() {
                    let mut eng = engine::Engine::new();
                    eng.TSTEP = *timestep;
                    eng.TSTOP = 2.0e-3;

                    let specs = format!("{:03} {} {} {} {}", i, timestep, amp, freq, isat);
                    println!("LOOP-SPEC {}", specs);
                    let ckt = build(*amp, *freq, *isat);

                    let filename = format!("waves/test_trans_ir_bridge_loaded_loop/{:03}.dat", i);
                    let stats = eng.transient_analysis(&ckt, &filename);
                    println!("{}", stats);
                    if stats.end >= eng.TSTOP {
                        println!("LOOP-RESULT {} GOOD\n\n", specs);
                    } else {
                        println!("LOOP-RESULT {} BAD\n\n", specs);
                        fails += 1;
                    }

                    i += 1;
                }
            }
        }
    }
    assert!(fails == 0);
}


fn build(amp: f64, freq: f64, isat: f64) -> Circuit {
    let mut ckt = Circuit::new();

    // bridge input voltage
    ckt.elements.push(
        Element::Isin(CurrentSourceSine{p: 0, n: 1, vo: 0.0, va: amp, freq: freq}),
    );
    ckt.elements.push(
        Element::R(Resistor{a: 1, b: 0, value: 10.0}),
    );


    ckt.elements.push(Element::V(VoltageSource{p: 2, n: 0, value: 0.0}));

    // Diode bridge
    //  (1) is top
    //  (2) is bottom
    ckt.elements.push( Element::D(Diode::new(1, 3, isat, 27.0) ) );
    ckt.elements.push( Element::D(Diode::new(4, 1, isat, 27.0) ) );
    ckt.elements.push( Element::D(Diode::new(2, 3, isat, 27.0) ) );
    ckt.elements.push( Element::D(Diode::new(4, 2, isat, 27.0) ) );

    // load
    ckt.elements.push( Element::R(Resistor{a: 3, b: 4, value: 1000.0}) );

    ckt
}

