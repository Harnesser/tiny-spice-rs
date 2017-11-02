use std::fmt;

pub enum Kind {
    DcOperatingPoint,
    DcSweep,
    Transient,
}

impl fmt::Display for Kind {
    fn fmt (&self, f:&mut fmt::Formatter) -> fmt::Result {
        match *self {
            Kind::Transient => write!(f, "Transient"),
            Kind::DcOperatingPoint => write!(f, "DC Operating Point"),
            Kind::DcSweep => write!(f, "DC Sweep"),
        }
    }
}

 

pub struct Statistics {
    pub kind: Kind,
    pub end: f64,
    pub iterations: usize,
}

impl fmt::Display for Statistics {
    fn fmt (&self, f:&mut fmt::Formatter) -> fmt::Result {
        let mut msg = format!("ANALYSIS: {}\n", self.kind);
        msg += format!("  Ended: {}\n", self.end).as_ref();
        msg += format!("  Iterations: {}\n", self.iterations).as_ref();
        write!(f, "{}", msg)
    }

}
