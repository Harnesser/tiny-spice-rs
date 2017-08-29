extern crate tiny_spice;

use tiny_spice::engine;

fn main() {
    engine::banner();

    println!("*INFO* Initialising");
    let eng = engine::Engine::new();


    println!("*INFO* Done");
}
