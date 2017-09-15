/// Waveform writer routines

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::error::Error;

pub struct WaveWriter {
    file: Option<File>,
}

impl WaveWriter {

    pub fn new(filename: &str) -> Option<WaveWriter> {

        // open file
        let path = Path::new(filename);
        let display = path.display();

        let mut writer = WaveWriter {
            file: None,
        };

        // open the path to write
        writer.file = match File::create(&path) {
            Err(why) => {
                println!("*ERROR* Can't open waveform file {}: {}",
                         display, why.description() );
                None
            },
            Ok(file) => Some(file),
        };

        Some(writer)
    }

    pub fn dump_vector(&mut self, time: f32, vars: &Vec<f32>) {
        if let Some(ref mut file) = self.file {
            let mut line = format!("{}", time);

            for var in vars {
                let bit = format!("\t{}", var);
                line += &bit;
            }
            line += "\n";
            let _ = file.write_all(line.as_bytes());
        }

    }

}


