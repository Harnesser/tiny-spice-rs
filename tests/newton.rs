extern crate tiny_spice;

use tiny_spice::diode::Diode;
use tiny_spice::newton_raphson::{Differentiable, DifferentiableEqn};

struct Line {
    pub m: f32,
    pub c: f32,
}

impl Differentiable for Line {
    fn eval(&self, x: f32) -> f32 {
        ( self.m * x ) + self.c
    }

    fn slope(&self, x: f32) -> f32 {
        self.m
    }
}


// Resistor and Current Source

fn resistor_isrc() -> DifferentiableEqn {

    let i1 = Line {
        m: 0.0,
        c: -2.0,
    };

    let r1 = Line{
        m: 1.0/3.0 ,
        c: 0.0,
    };

    let mut cde = DifferentiableEqn {
        eqns: vec![],
    };

    cde.eqns.push(Box::new(i1));
    cde.eqns.push(Box::new(r1));
    cde
}


#[test]
fn basic_r_solve() {
    let cde = resistor_isrc();
    let answer = cde.solve(1.0);
    assert!(answer == Some(6.0), "answer was {:?}", answer);
}


//
// Diode and current source
// Keep current source low, so that diode can sink it alone
//

fn diode_isrc() -> DifferentiableEqn {

    let d1 = Diode {
        tdegc: 27.0,
        i_sat: 1.0e-9,
    };

    let i1 = Line {
        m: 0.0,
        c: -0.001,
    };

    let mut cde = DifferentiableEqn {
        eqns: vec![],
    };

    cde.eqns.push(Box::new(i1));
    cde.eqns.push(Box::new(d1));
    cde
}

#[test]
fn basic_eval_0p1() {
    let cde = diode_isrc();
    let answer = cde.eval(0.1);
    assert!(answer == 0.0, "Answer was {}", answer);
}

#[test]
fn basic_eval_0p6() {
    let cde = diode_isrc();
    let answer = cde.eval(0.6);
    assert!(answer == 0.0, "Answer was {}", answer);
}

#[test]
fn basic_slope() {
    let cde = diode_isrc();
    let answer = cde.slope(0.1);
    assert!(answer == 0.0, "Answer was {}", answer);
}

#[test]
fn basic_solve_0p3() {
    let v_0 = 0.3;
    let cde = diode_isrc();

    let i_0 = cde.eqns[1].eval(v_0);
    println!("*INFO* Initial diode current Vd = {}, Id = {}", v_0, i_0);
    let answer = cde.solve(v_0);
    assert!(answer == Some(0.0), "Answer was {:?}", answer);
}

#[test]
fn basic_solve_0p7() {
    let v_0 = 0.7;
    let cde = diode_isrc();

    let i_0 = cde.eqns[1].eval(v_0);
    println!("*INFO* Initial diode current Vd = {}, Id = {}", v_0, i_0);
    let answer = cde.solve(v_0);
    assert!(answer == Some(0.0), "Answer was {:?}", answer);
}

#[test]
fn basic_solve_eval() {
    let cde = diode_isrc();
    let answer = cde.solve(0.1);
    let reeval = cde.eval(answer.unwrap());
    assert!(reeval == 0.0, "reeval was {:?}", reeval);
}


fn diode_resistor_isrc() -> DifferentiableEqn {

    let alpha = 0.001 / 0.501;

    let d1 = Diode {
        tdegc: 27.0,
        i_sat: 1.0e-9,
    };

    let i1 = Line {
        m: 0.0,
        c: 3.0 * alpha,
    };

    let r1 = Line {
        m: ( -0.001 * alpha ) + 0.001 ,
        c: 0.0,
    };

    let mut cde = DifferentiableEqn {
        eqns: vec![],
    };

    cde.eqns.push(Box::new(i1));
    cde.eqns.push(Box::new(d1));
    cde.eqns.push(Box::new(r1));
    cde
}

//#[test]
fn plot_diode() {

    let d1 = Diode {
        tdegc: 27.0,
        i_sat: 1.0e-9,
    };

    for i in 0..500 {
        let v = i as f32 / 100.0;
        let i = d1.eval(v);
        println!("{} {}", v, i);
    }

    assert!(false);
}

