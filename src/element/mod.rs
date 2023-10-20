//! Circuit Elements (or Devices)

use std::fmt;

pub mod resistor;
pub mod capacitor;
pub mod diode;

pub mod isine;
pub mod vsine;

pub mod vpwl;

pub mod independent;

pub mod vdepsrc;

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
    Vpwl(vpwl::VoltageSourcePwl),
    C(capacitor::Capacitor),
    Vcvs(vdepsrc::Vcvs),
    Vccs(vdepsrc::Vccs),
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
            Element::Vpwl(ref el) => {
                write!(f, "Vpwl p:{} n:{}",
                    el.p, el.n)
            },
            Element::C(ref el) => {
                write!(f, "C a:{} b:{} {} Farads ({})",
                    el.a, el.b, el.value, el.ident)
            },
            Element::Vcvs(ref el) => {
                write!(f, "VCVS p:{} n:{} cp:{} cn:{} k={} ({})",
                    el.p, el.n, el.cp, el.cn, el.k, el.ident)
            },
            Element::Vccs(ref el) => {
                write!(f, "VCCS p:{} n:{} cp:{} cn:{} k={} ({})",
                    el.p, el.n, el.cp, el.cn, el.k, el.ident)
            },
        }
    }
}
