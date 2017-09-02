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

    fn slope(&self, _: f32) -> f32 {
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
// Diode and Norton Source
//
fn diode_resistor_isrc() -> DifferentiableEqn {
    let r = 2.0;
    let i = 5.0;

    let d1 = Diode {
        tdegc: 27.0,
        i_sat: 1.0e-9,
    };

    let i1 = Line {
        m: 0.0,
        c: -i,
    };

    let r1 = Line {
        m: 1.0/r,
        c: 0.0,
    };

    let gmin = Line {
        m: 1.0e-12,
        c: 0.0,
    };

    let mut cde = DifferentiableEqn {
        eqns: vec![],
    };

    cde.eqns.push(Box::new(i1));
    cde.eqns.push(Box::new(d1));
    cde.eqns.push(Box::new(r1));
    //cde.eqns.push(Box::new(gmin));
    cde
}


#[test]
fn basic_solve_0p3() {
    let v_0 = 0.3;
    let cde = diode_resistor_isrc();

    let i_0 = cde.eqns[1].eval(v_0);
    println!("\n*INFO* Initial diode current Vd = {}, Id = {}", v_0, i_0);
    let answer = cde.solve(v_0);
    assert!(answer == Some(0.0), "Answer was {:?}", answer);
}

#[test]
fn basic_solve_0p8() {
    let v_0 = 0.8;
    let cde = diode_resistor_isrc();

    let i_0 = cde.eqns[1].eval(v_0);
    println!("\n*INFO* Initial diode current Vd = {}, Id = {}", v_0, i_0);
    let answer = cde.solve(v_0);
    assert!(answer == Some(0.0), "Answer was {:?}", answer);
}

#[allow(dead_code)]
//#[test]
fn basic_solve_eval() {
    let cde = diode_resistor_isrc();
    let answer = cde.solve(0.1);
    let reeval = cde.eval(answer.unwrap());
    assert!(reeval == 0.0, "reeval was {:?}", reeval);
}



#[allow(dead_code)]
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

