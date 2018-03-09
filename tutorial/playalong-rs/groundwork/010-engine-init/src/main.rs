extern crate tiny_spice;

use tiny_spice::engine;

fn banner(build: &str) {
    println!("=======================================================");
    println!(" tiny-spice - a Toy SPICE electrical circuit simulator");
    println!("                  (c) CrapCorp");
    println!(" all rights not reserved");
    println!(" no patents pending");
    println!(" build: {}", build);
    println!("=======================================================");
}

fn main() {
    banner("000");
    engine::init();
}
