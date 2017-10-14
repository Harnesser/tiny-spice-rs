extern crate tiny_spice;

use tiny_spice::circuit::*;
use tiny_spice::engine;

#[test]
#[allow(non_snake_case)]
fn test_trans_ir_bridge_1kHz_10us() {

    let mut eng = engine::Engine::new();
    eng.TSTEP = 10e-6;
    let ckt = build_old(2.0, 1.0e3);
    let stats = eng.transient_analysis(&ckt, "waves/trans_ir_bridge_1kHz_10us.dat");
    println!("\n*INFO* Done");
    println!("{}", stats);
    assert!(stats.end >= eng.TSTOP);
}


#[test]
#[ignore]
#[allow(non_snake_case)]
fn test_trans_ir_bridge_1kHz_1us() {

    let mut eng = engine::Engine::new();
    eng.TSTEP = 1.0e-6;
    let ckt = build_old(2.0, 1.0e3);
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

    let mut i = 0;
    let mut fails = 0;
    for timestep in timesteps.iter() {
        for freq in freqs.iter() {
            for amp in amps.iter() {
                let mut eng = engine::Engine::new();
                eng.TSTEP = *timestep;
                eng.TSTOP = 2.0e-3;
                let ckt = build_old(*amp, *freq);

                let filename = format!("waves/test_trans_ir_bridge_loaded_loop/{:03}.dat", i);
                let stats = eng.transient_analysis(&ckt, &filename);
                println!("{}", stats);
                if stats.end >= eng.TSTOP {
                    println!("LOOPRESULT {} {} {} GOOD\n\n", timestep, amp, freq);
                } else {
                    println!("LOOPRESULT {} {} {} BAD\n\n", timestep, amp, freq);
                    fails += 1;
                }

                i += 1;
            }
        }
    }
    assert!(fails == 0);
}


#[allow(dead_code)]
fn build(freq: f32) -> Circuit {
    let mut ckt = Circuit::new();

    // bridge input voltage
    //ckt.elements.push(Element::V(VoltageSource{p: 1, n: 2, value: 10.0}));
    ckt.elements.push(
        Element::Isin(CurrentSourceSine{p: 0, n: 1, vo: 0.0, va: 2.0, freq: freq}),
    );
    ckt.elements.push(
        Element::R(Resistor{a: 1, b: 0, value: 10.0}),
    );


    // Diode bridge
    //  (1) is top
    //  (2) is bottom
    ckt.elements.push( Element::D(Diode{p: 1, n: 3, i_sat: 1e-9, tdegc: 27.0}) );
    //ckt.elements.push( Element::D(Diode{p: 2, n: 1, i_sat: 1e-9, tdegc: 27.0}) );
    //ckt.elements.push( Element::D(Diode{p: 0, n: 3, i_sat: 1e-9, tdegc: 27.0}) );
    ckt.elements.push( Element::D(Diode{p: 2, n: 0, i_sat: 1e-9, tdegc: 27.0}) );

    // load
    ckt.elements.push( Element::R(Resistor{a: 3, b: 2, value: 1000.0}) );

    ckt
}


#[allow(dead_code)]
fn build_old(amp: f32, freq: f32) -> Circuit {
    let mut ckt = Circuit::new();
    const I_SAT:f32 = 1e-9;
    // bridge input voltage
    //ckt.elements.push(Element::V(VoltageSource{p: 1, n: 2, value: 10.0}));
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
    ckt.elements.push( Element::D(Diode{p: 1, n: 3, i_sat: I_SAT, tdegc: 27.0}) );
    //ckt.elements.push( Element::D(Diode{p: 4, n: 1, i_sat: I_SAT, tdegc: 27.0}) );
    //ckt.elements.push( Element::D(Diode{p: 2, n: 3, i_sat: I_SAT, tdegc: 27.0}) );
    ckt.elements.push( Element::D(Diode{p: 4, n: 2, i_sat: I_SAT, tdegc: 27.0}) );

    // load
    ckt.elements.push( Element::R(Resistor{a: 3, b: 4, value: 1000.0}) );

    ckt
}

