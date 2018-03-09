Getting Started
======================================================================

First, we'll get started with the toolchains: Rust or Python. Depending
on what you're following along with, follow one of the sets of 
instructions below.

In fact, I should just call out to other pages on the internet, most 
likely Rust and Cargo websites, to set stuff up.


## Software Tools

You'll need one of either Rust or Python, and possibly Git depending what
your learning objectives are for this project.


### Rust & Cargo
Download Rust and Cargo.

Rust is the programming language. Cargo is the software tool used to 
launch the programs.

Follow instructions [here][rustup]

  [rustup]: https://www.rust-lang.org/en-US/install.html


Then follow the instructions [here][rust_hello], to start a new project

  [rust_hello]: https://www.rust-lang.org/en-US/install.html


Questions:
1. Should I try to stay on stable? Probably.



### Python

Use `pip` and `virtualenv`? 

Questions:
1. What's the equivalent of "hello, world!" in the Python world? 
2. Should I point towards Python3?


### Git


### NgSPICE

A SPICE simulator to compare ourselves against. 


### First Edit
It's vitally important that we start out on the right footing and create a banner
for our new computer-aided design (CAD) software company. 


Here's mine:

    fn banner(build: &str) {
        println!("=======================================================");
        println!(" tiny-spice - a Toy SPICE electrical circuit simulator");
        println!("                  (c) CrapCorp");
        println!(" all rights not reserved");
        println!(" no patents pending");
        println!(" build: {}", build);
        println!("=======================================================");
    }


Add this to `src/main.rs` before the definition of `main()`. Then call it from main:

    fn main() {
        banner("000");
    }
 
