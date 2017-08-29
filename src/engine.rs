

pub fn banner() {

    println!("********************************************");
    println!("***       Tiny-SPICE-Simulator           ***");
    println!("***        (c) CrapCorp 2017             ***");
    println!("*** Patent Pending, All rights reserved  ***");
    println!("********************************************");

}

pub struct Engine {
    next_id: usize,
}

impl Engine {

    pub fn new() -> Engine {
        Engine {
            next_id: 1,
        }
    }

}

