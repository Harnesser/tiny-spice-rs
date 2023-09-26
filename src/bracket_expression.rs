//! Bracket Expressions
//!
//! Datastructor for `{2*cval}` or whatever

use std::fmt;

/// Program execution trace macro - prefix `<bktexpr>`
macro_rules! trace {
    ($fmt:expr $(, $($arg:tt)*)?) => {
        // uncomment the line below for tracing prints
        println!(concat!("<bktexpr> ", $fmt), $($($arg)*)?);
    };
}

/// Expression
#[derive(Clone, Debug)]
pub enum Expression {
    Literal(f64),
    Identifier(String),
}

impl fmt::Display for Expression {
    fn fmt (&self, f:&mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expression::Literal(ref p) => {
                write!(f, "{}", p)
            },
            Expression::Identifier(ref p) => {
                write!(f, "Identifier({})", p)
            },
        }
    }
}


/// Extract a bracket expression
///
/// FIXME - allow `_` too...
pub fn extract_expression(text: &str) -> Option<Expression> {

    if !text.starts_with('{') {
        let val = extract_value(text);
        if let Some(n) = val {
            return Some(Expression::Literal(n));
        } else {
            println!("*ERROR* expected numerical literal in expression");
            return None;
        }
    }

    println!("*WARN* only expressions that are a single identifier are supported");

    let expr_str: Vec<_> = text.chars().collect();

    let mut i:usize = 0;

    trace!("looking for '{{'");
    if expr_str[i] == '{' {
        i += 1
    } else {
        println!("*ERROR* - expected '{{'");
        return None
    }

    while expr_str[i].is_whitespace() {
        i += 1;
        if i >expr_str.len() {
            return None;
        }
    }

    trace!("looking for identifier");
    let mut ident_chars: Vec<char> = vec![];
    while expr_str[i].is_ascii_alphanumeric() {
        ident_chars.push(expr_str[i]);
        i += 1;
        if i > expr_str.len() {
            return None;
        }
    }

    let ident: String = ident_chars.into_iter().collect();

    trace!("looking for '}}'");
    while expr_str[i].is_whitespace() {
        i += 1;
        if i > expr_str.len() {
            return None;
        }
    }

    if expr_str[i] == '}' {
    } else {
        println!("*ERROR* - expected '}}'");
        return None
    }

    trace!("Identifier: '{}'", ident);

    Some(Expression::Identifier(ident))
}


/// Extract an element identifier from SPICE
// Just take the entire thing as an identifier
pub fn extract_identifier(text: &str) -> String {
    text.to_string()
}

#[derive(Debug)]
enum ValueState {
    Start,
    Int,
    Frac,
    ExpStart, // '+' | '-' | digit
    Exp, // digit
    Unit,
}

/// Extract a value possibly in engineering notation from a lump of text.
///
/// Supported engineering: k m u n p (future: meg f)
///
/// Some suported examples
/// * 10
/// * 10.0
/// * 10.0m
/// * 10mA
///
/// Some Unsupported examples
/// * 10.0megV [not implemented]
/// * 10.0e-6 [not implemented]
/// * 10.0e-6V [not implemented]
/// * 10.0meg [not implemented]
///
/// Unsupported engineering: meg f
///
/// Unsupported units: them all, e.g. A V F s
pub fn extract_value(text: &str) -> Option<f64> {
    let mut value: Option<f64> = None;
    let mut float_str = "".to_string();
    let mut c: char;
    let mut state = ValueState::Start;
    let mut nxt;
    let mut eng_mult :f64 = 1.0;

    //println!("VALUE: '{}'", text);
    let mut text_iter = text.chars();

    fn eval( txt :&str, mult: f64) -> Option<f64> {
        Some( txt.parse::<f64>().unwrap() * mult )
    }

    'things: loop {

        if let Some(c_) = text_iter.next() {
            c = c_;
        } else {
            break 'things;
        }
        //println!(" {:?} '{}'", state, c);
        match state {

            ValueState::Start => {
                match c {
                    '+' | '-' => { float_str.push(c); nxt = ValueState::Int },
                    '0' ..= '9' => { float_str.push(c); nxt = ValueState::Int },
                    _ => break 'things
                }
            },

            ValueState::Int => {
                match c {
                    '0' ..= '9' => { float_str.push(c); nxt = ValueState::Int },
                    '.' => { float_str.push(c); nxt = ValueState::Frac },
                    'e' => { float_str.push(c); nxt = ValueState::ExpStart },
                    'k' => {
                        eng_mult = 1e3;
                        value = eval(&float_str, eng_mult);
                        nxt = ValueState::Unit
                    },
                    'm' => {
                        eng_mult = 1e-3;
                        value = eval(&float_str, eng_mult);
                        nxt = ValueState::Unit
                    },
                    'u' => {
                        eng_mult = 1e-6;
                        value = eval(&float_str, eng_mult);
                        nxt = ValueState::Unit
                    },
                    'n' => {
                        eng_mult = 1e-9;
                        value = eval(&float_str, eng_mult);
                        nxt = ValueState::Unit
                    },
                    'p' => {
                        eng_mult = 1e-12;
                        value = eval(&float_str, eng_mult);
                        nxt = ValueState::Unit
                    },
                    _ => break 'things
                }
            },

            ValueState::Frac => {
                match c {
                    '0' ..= '9' => { float_str.push(c); nxt = ValueState::Frac },
                    'e' => { float_str.push(c); nxt = ValueState::ExpStart },
                    'k' => {
                        eng_mult = 1e3;
                        value = eval(&float_str, eng_mult);
                        nxt = ValueState::Unit
                    },
                    'm' => {
                        eng_mult = 1e-3;
                        value = eval(&float_str, eng_mult);
                        nxt = ValueState::Unit
                    },
                    'u' => {
                        eng_mult = 1e-6;
                        value = eval(&float_str, eng_mult);
                        nxt = ValueState::Unit
                    },
                    'n' => {
                        eng_mult = 1e-9;
                        value = eval(&float_str, eng_mult);
                        nxt = ValueState::Unit
                    },
                    'p' => {
                        eng_mult = 1e-12;
                        value = eval(&float_str, eng_mult);
                        nxt = ValueState::Unit
                    },
                    _ => break 'things
                }
            },

            ValueState::ExpStart => {
                match c {
                    '+' | '-' => { float_str.push(c); nxt = ValueState::Exp },
                    '0' ..= '9' => { float_str.push(c); nxt = ValueState::Exp },
                    _ => break 'things
                }
            },

            ValueState::Exp => {
                match c {
                    '0' ..= '9' => { float_str.push(c); nxt = ValueState::Exp },
                    _ => break 'things
                }
            },

            ValueState::Unit => {
                break 'things
            },
        }

        //println!(" -> {:?} '{}'", nxt, float_str);
        state = nxt;
    }

    // if we've broken out of the loop at a point where the gathered
    // string might be a valid number, calculate it.
    match state {
        ValueState::Int | ValueState::Frac | ValueState::Exp => {
            value = eval(&float_str, eng_mult)
        },
        _ => {}
    }

    value
}


