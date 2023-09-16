//! Waveform writer routines
//!
//! Routines to open a file and dump all the data from the current timestep
//! into it.

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

// for handle to node id lookup table
use std::collections::HashMap;
use circuit::NodeId;

pub struct WaveWriter<'hash>{
    file: Option<File>,
    lut: &'hash HashMap<NodeId, String>,
}

impl WaveWriter<'_> {

    pub fn new<'hash>(filename: &str, lut: &'hash HashMap<NodeId, String>) 
        -> Option<WaveWriter<'hash>>
    {

        // open file
        let path = Path::new(filename);
        let display = path.display();

        // make all parent directories
        let leadup = path.parent().expect("Can't get path bit of waveform file");
        std::fs::create_dir_all(leadup).expect("Can't make waveform directory");

        // wave writer
        let mut writer = WaveWriter {
            file: None,
            lut,
        };

        // open the path to write
        writer.file = match File::create(path) {
            Err(why) => {
                println!("*ERROR* Can't open waveform file {}: {}",
                         display, why );
                None
            },
            Ok(file) => {
                println!("*INFO* Dumping into file {}", display);
                Some(file)
            },
        };

        Some(writer)
    }


    pub fn header(&mut self, c_nodes: usize, c_vsrcs: usize) {

        let mut names = "Time".to_string();

        for i in 0..c_nodes {
            let name = &self.lut[&i];
            names += &format!("\tv({})", name);
        }
        for j in 0..c_vsrcs {
            names += &format!("\ti({})", j);
        }
        names += "\n";

        let mut units = "s".to_string();
        for _ in 0..c_nodes {
            units += "\tV";
        }
        for _ in 0..c_vsrcs {
            units += "\tA";
        }
        units += "\n";

        if let Some(ref mut file) = self.file {
            let _ = file.write_all(names.as_bytes());
            let _ = file.write_all(units.as_bytes());
        }
    }

    pub fn dump_vector(&mut self, time: f64, vars: &[f64]) {
        if let Some(ref mut file) = self.file {
            let mut line = format!("{:0.9}", time);

            for var in vars {
                let bit = format!("\t{:0.9}", var);
                line += &bit;
            }
            line += "\n";
            let _ = file.write_all(line.as_bytes());
        }

    }

}


