/* "integration" tests
 *
 * after  ripgrep/tests/tests.rs 
 */


use std::process::Command;


macro_rules! spice {

    ($name:ident, $path: expr) => {
        #[test]
        fn $name() {
            let mut cmd = Command::new("target/debug/tiny-spice-rs");
            cmd.arg($path);
            let output = cmd.output().expect("failed to execute");

            assert!(output.status.success());
        }

    };

}


// cargo test spice - will run all tests starting with "spice"

// check out some number format parsing
spice!(spice_reader, "./ngspice/test_reader.spi");

// DC solve of a resistor network
spice!(spice_irrrr, "./ngspice/test_irrrr.spi");

// Transient analysis of a HPF
spice!(spice_irrc, "./ngspice/test_irrc.spi");

