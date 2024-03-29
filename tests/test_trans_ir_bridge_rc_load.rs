extern crate tiny_spice;

use tiny_spice::circuit::*;
use tiny_spice::engine;
use tiny_spice::analysis;

#[test]
#[allow(non_snake_case)]
fn test_trans_ir_bridge_rc_1kHz() {

    let mut eng = engine::Engine::new();
    let mut cfg = analysis::Configuration::new();

    cfg.set_transient(2.0e-3, 1e-6, 0.0);
    cfg.set_wavefile("waves/trans_ir_bridge_1kHz_rc_load.dat");

    let ckt = build(2.0, 1e3, 1e-6);
    let stats = eng.transient_analysis(&ckt, &cfg);
    println!("\n*INFO* Done");
    println!("{}", stats);
    assert!(stats.end >= cfg.TSTOP);
}


#[test]
fn test_trans_ir_bridge_rc_failure_003() {

    let mut eng = engine::Engine::new();
    let mut cfg = analysis::Configuration::new();

    cfg.set_transient(2.0e-3, 0.000001, 0.0);
    cfg.set_wavefile("waves/trans_ir_bridge_rc_failure_003.dat");
    let ckt = build(-2.0, 3.0e3, 0.000001);
    let stats = eng.transient_analysis(&ckt, &cfg);
    println!("\n*INFO* Done");
    println!("{}", stats);
    assert!(stats.end >= cfg.TSTOP);
}


#[test]
#[ignore]
#[allow(non_snake_case)]
fn test_trans_ir_bridge_rc_load_loop() {

    let timesteps = [10e-6, 5e-6, 2e-6, 1e-6];
    let amps = [-2.0, -1.0, -0.5, 0.5, 1.0, 2.0];
    let freqs = [3.0e3, 1.0e3, 0.5e3, 0.4e3, 0.3e3];
    let caps = [1e-6, 100e-6];
/*

    let timesteps = [1e-6];
    let amps = [-2.0, 2.0];
    let freqs = [3.0e3, 1.0e3];
    let caps = [1e-6, 100e-6];
*/

    let mut i = 0;
    let mut fails = 0;
    for cap in caps.iter() {
        for freq in freqs.iter() {
            for amp in amps.iter() {
                for timestep in timesteps.iter() {
                    let mut eng = engine::Engine::new();
                    let mut cfg = analysis::Configuration::new();
                    cfg.set_transient(2.0e-3, *timestep, 0.0);

                    let specs = format!("{:03} {} {} {} {}", i, timestep, amp, freq, cap);
                    println!("LOOP-SPEC {}", specs);
                    let ckt = build(*amp, *freq, *cap);

                    let filename = format!("waves/test_trans_ir_bridge_rc_load_loop/{:03}.dat", i);
                    cfg.set_wavefile(&filename);
                    let stats = eng.transient_analysis(&ckt, &cfg);
                    println!("{}", stats);
                    if stats.end >= cfg.TSTOP {
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




#[allow(dead_code)]
fn build(amp: f64, freq: f64, cap: f64) -> Circuit {
    let isat = 1e-12;
    let mut ckt = Circuit::new();

    ckt.add_node("1");
    ckt.add_node("2");
    ckt.add_node("3");
    ckt.add_node("4");

    // bridge input voltage
    //ckt.elements.push(Element::V(VoltageSource{p: 1, n: 2, value: 10.0}));
    ckt.elements.push(
        Element::Isin(CurrentSourceSine{p: 0, n: 1, vo: 0.0, va: amp, freq: freq}),
    );
    ckt.elements.push(
        Element::R(Resistor{ident: "r1".to_string(), a: 1, b: 0, value: 10.0}),
    );


    ckt.elements.push(Element::V(VoltageSource{p: 2, n: 0, value: 0.0, idx:0}));

    // Diode bridge
    //  (1) is top
    //  (2) is bottom
    ckt.elements.push( Element::D(Diode::new("D1", 1, 3, isat, 27.0)) );
    ckt.elements.push( Element::D(Diode::new("D2", 4, 1, isat, 27.0)) );
    ckt.elements.push( Element::D(Diode::new("D3", 2, 3, isat, 27.0)) );
    ckt.elements.push( Element::D(Diode::new("D4", 4, 2, isat, 27.0)) );

    let c_diode = 1e-12;
    ckt.elements.push( Element::C(Capacitor::new("C1", 1, 3, c_diode)) );
    ckt.elements.push( Element::C(Capacitor::new("C2", 4, 1, c_diode)) );
    ckt.elements.push( Element::C(Capacitor::new("C3", 2, 3, c_diode)) );
    ckt.elements.push( Element::C(Capacitor::new("C4", 4, 2, c_diode)) );

    // load
    ckt.elements.push( Element::R(Resistor{ident: "r2".to_string(), a: 3, b: 4, value: 1000.0}) );
    ckt.elements.push( Element::C(Capacitor{ident: "c199".to_string(), a: 3, b: 4, value: cap}) );

    ckt.build_node_id_lut();
    ckt
}

