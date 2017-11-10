use std::fmt;

#[derive(Clone)]
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

/// Analysis datastructure holding all of the options such as
/// RELTOL, ITL4, etc.
#[derive(Clone)]
#[allow(non_snake_case)]
pub struct Configuration {

    /// What kind of analysis to run: DC op, transient, etc,
    pub kind: Option<Kind>,

    /// Minimum conductance between nodes in the design
    pub GMIN: f64,

    /// Ambient temperature in Celcius
    pub TDEGC: f64,

    /// Relative tolerance target for convergence
    pub RELTOL: f64,

    /// Voltage absolute tolerance
    pub VNTOL: f64,

    /// Current absolute tolerance
    pub ABSTOL: f64,

    /// Newton solver maximum number of iterations
    pub ITL1: usize,

    /// Start time for transient analysis output recording
    pub TSTART: f64,

    /// Stop time for transient analysis
    pub TSTOP: f64,

    /// Step time for transient analysis output recording
    pub TSTEP: f64,

    /// Initial timestep factor for transient analysis
    pub FS: f64,

    /// Timestep adjustment factor on iteration failure for transient analysis
    pub FT: f64,

    /// Smallest delta-time step allowed = RMIN * TSTEP for transient analysis
    pub RMIN: f64,

    /// Largest delta-time step allowed factor
    pub RMAX: f64,

    /// 'Easy' iteratin count limit for transient analysis.
    /// If we solve in fewer iterations, increase delta-time.
    pub ITL3: usize,

    /// 'Struggling' iteration count for transient analysis.
    /// If this count is reached before a solution is found for the current time
    /// step, reduce the delta-time step and restart the solution attempt.
    pub ITL4: usize,

    /// Name of file to write waveform data to
    pub wavefile: String,
}


impl Configuration {

    /// Create a new `Analysis` datastructure with default settings for
    /// all options.
    pub fn new() -> Configuration {
        Configuration {
            kind: None,

            // General
            GMIN : 1.0e-12,
            TDEGC: 27.0,

            // Cnvergence
            RELTOL: 0.0001,
            VNTOL: 1.0e-6,
            ABSTOL: 1.0e-9,

            // DC operating
            ITL1: 50,

            // Transient
            TSTART: 0.0,
            TSTOP: 1e-3,
            TSTEP: 1e-6,

            FS: 0.25,
            FT: 0.25,
            RMIN: 1e03,
            RMAX: 5.0,
            ITL3: 6,
            ITL4: 50,

            wavefile: "waves/default.dat".to_string(),

        }
    }


    // Configure the simulation engine for a transient analysis
    pub fn set_transient(&mut self, tstop: f64, tstep: f64, tstart: f64) {
        self.kind = Some(Kind::Transient);
        self.TSTOP = tstop;
        self.TSTEP = tstep;
        self.TSTART = tstart;
    }

    // Configure the simulation engine for a DC operating point analysis
    pub fn set_dc_operating_point(&mut self) {
        self.kind = Some(Kind::DcOperatingPoint);
    }

    // name file for writing waveforms to
    pub fn set_wavefile(&mut self, filename: &str) {
        self.wavefile = filename.to_string();
    }



}

