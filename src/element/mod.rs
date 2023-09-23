//! Circuit Elements (or Devices)

use std::fmt;

pub mod resistor;
pub mod capacitor;
pub mod diode;

pub mod isine;
pub mod vsine;

pub mod independent;

/// Circuit Elements that this simulator supports
#[allow(dead_code)]
#[derive(Clone)]
pub enum Element {
    R(resistor::Resistor),
    I(independent::CurrentSource),
    V(independent::VoltageSource),
    D(diode::Diode),
    Isin(isine::CurrentSourceSine),
    Vsin(vsine::VoltageSourceSine),
    C(capacitor::Capacitor),
}


impl fmt::Display for Element {
    fn fmt (&self, f:&mut fmt::Formatter) -> fmt::Result {
        match *self {
            Element::I(ref el) => {
                write!(f, "I p:{} n:{} {} A", el.p, el.n, el.value)
            },
            Element::R(ref el) => {
                write!(f, "R a:{} b:{} {} Ohms ({})",
                    el.a, el.b, el.value, el.ident)
            },
            Element::V(ref el) => {
                write!(f, "V a:{} b:{} {} Volts", el.p, el.n, el.value)
            },
            Element::D(ref el) => {
                write!(f, "D p:{} n:{} I_sat={} A ({})",
                    el.p, el.n, el.i_sat, el.ident)
            },
            Element::Isin(ref el) => {
                write!(f, "Isin p:{} n:{} = {} + {} * sin(2pi {})",
                    el.p, el.n, el.vo, el.va, el.freq)
            },
            Element::Vsin(ref el) => {
                write!(f, "Vsin p:{} n:{} = {} + {} * sin(2pi {})",
                    el.p, el.n, el.vo, el.va, el.freq)
            },
            Element::C(ref el) => {
                write!(f, "C a:{} b:{} {} Farads ({})",
                    el.a, el.b, el.value, el.ident)
            },
        }
    }
}
