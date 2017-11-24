/* "integration" tests
 *
 * after  ripgrep/tests/tests.rs 
 */


use std::process::Command;


macro_rules! spice {

    ($name:ident, $path: expr) => {
        #[test]
        fn $name() {
            let mut cmd = Command::new("target/debug/tiny-spice");
            cmd.arg($path);
            let mut child = cmd.spawn().expect("failed to execute");

            let ecode = child.wait().expect("failed to wait");
            assert!(ecode.success());
        }

    };

}


// cargo test spice - will run all tests starting with "spice"
spice!(spice_irrrr, "./ngspice/test_reader.spi");

