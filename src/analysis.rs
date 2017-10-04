use std::fmt;

pub enum Kind {
    DC_Operating_Point,
    DC_Sweep,
    Transient,
}

impl fmt::Display for Kind {
    fn fmt (&self, f:&mut fmt::Formatter) -> fmt::Result {
        match *self {
            ref Transient => write!(f, "Transient"),
            ref DC_Sweep => write!(f, "DC Sweep"),
            ref DC_Operating_Point => write!(f, "DC Operating Point"),
        }
    }
}

 

pub struct Statistics {
    pub kind: Kind,
    pub end: f32,
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
