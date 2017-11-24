/* "integration" tests
 *
 * after  ripgrep/tests/tests.rs 
 */


use std::process::Command;


macro_rules! spice {

    ($name:ident, $path: expr) => {
        #[test]
        fn $name() {
            let mut cmd = Command::new("ls");
            cmd.arg("-lrt");
            cmd.spawn();
            assert!(false);
        }

    };

}


// cargo test spice - will run all tests starting with "spice"
spice!(spice_irrrr, "../ngspice/test_reader.spi");

