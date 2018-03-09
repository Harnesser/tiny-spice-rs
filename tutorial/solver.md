The Solver
==========================================================

We don't have to worry about stamping the matrix with component elements
just yet, but we can implement and test a Guassian Elimination solver. 
And we do this by cribbing massively off [wikipedia][GaEl-wiki-pseudo]:
we'll do row-reordering using partial pivoting, and then use back-substitution to
solve the variables. No biggie.

Our `Engine` struct feels like it has to have a method called `solve()` that
will do all of the above, and return a vector of solved values. Since there's
no guarantee that whatever numbers we stick into the matrix will represent a
solvable system of linear equations -- we have to legislate for failure. Not
being able to solve the matrix is kinda catastrophic for a circuit simulator,
so it's reasonable for `solve()` to return a [`Result`][rust-result] type
rather than an option:

    pub fn solve(&mut self) -> Result< Vec<f32>, &'static str> {
        ... a call to self.reorder(), say
	... a call to self.back_substitute() 
	// plus whatever error wrapper thing...
    }

The function is called on the structure this time (unlike `new()`) and we're
changing data in the structure, so `solve()` needs to take a mutable reference to
the structure. It will return a vector of results in an `Ok()` enum, or a
failure. All this might be a bit daunting if you're newish to Rust, maybe the
following testing will help things settle in place for you if you're struggling:

Let's add the Cargo testing structure to `src/engine.rs`. At the bottom of the
file:

    #[cfg(test)]
    mod tests {
    
        #[test]
        fn it_works() {
            assert_eq!(2,2);
        }
    
    }


Do a `cargo test` (`test` not `run` this time), and look for the following line in 
the output:

    test engine::tests::it_works ... ok

It's tres simple to get Rust's testing framework running, but the code above isn't
really doing anything for our simulator. Let's sketch out a first test - we'll again
rob from wikipedia, this time for a testcase:


    // #[cfg(test)]
    mod tests {
    
        use super::*; // magic so we can use `Engine`
    
        #[test]
        fn it_works() {
    
            let mut eng = Engine::new(3,0);
    
            // fill the matrix with the example from wikipedia:
            // https://en.wikipedia.org/wiki/Gaussian_elimination#Example_of_the_algorithm
            eng.m[0][0] = 2.0;
            eng.m[0][1] = 1.0;
            eng.m[0][2] = -1.0;
            eng.m[0][3] = 8.0; // augmented column
    
            eng.m[1][0] = -3.0;
            eng.m[1][1] = -1.0;
            eng.m[1][2] = 2.0;
            eng.m[1][3] = -11.0; // augmented column
    
            eng.m[2][0] = -2.0;
            eng.m[2][1] = 1.0;
            eng.m[2][2] = 2.0;
            eng.m[2][3] = 2.0; // augmented column
    
            println!("{:?}", eng.m);
    
            let res = eng.solve();
    
            assert_eq!(res, Ok(vec![2.0, 3.0, 1.0]) );
        }
    
    }

Oh yeah - I got the matrix dimensions wrong, did you spot it? Modify the line that
initialises the matrix in `src/engine.rs` to:

    m: vec![ vec![0.0; n_node+n_vsrc+1]; n_node+n_vsrc],

Note the `+1` - this gives us an extra column to put in the !!!FIXME!!! branch currents.

To sketch out the `solve()` function, write the following inside the `impl Engine` block
after the `::new()`:

    /// Solve the system of linear equations represented by the matrix
    ///
    /// Using Guassian Elimination: row reordering, then back-substitution
    pub fn solve(&mut self) -> Result< Vec<f32>, &'static str> {
        Ok( vec![0.0; self.n_vsrc + self.n_node] )
    }

The `solve()` function just returns a vector of `0.0`s, wrapped in an `Ok()`. Running
`cargo test` is unsucessful, unsurprisingly:


    ---- engine::tests::it_works stdout ----
    	*INFO* Initialising circuit matrix
    [[2, 1, -1, 8], [-3, -1, 2, -11], [-2, 1, 2, 2]]
    thread 'engine::tests::it_works' panicked at 'assertion failed: `(left == right)`
      left: `Ok([0, 0, 0])`,
     right: `Ok([2, 3, 1])`', src/engine.rs:74:8
    note: Run with `RUST_BACKTRACE=1` for a backtrace.




References
----------

  [MNA-QUCs]: http://qucs.sourceforge.net/tech/node14.html
  [MNA-wiki]: https://en.wikipedia.org/wiki/Modified_nodal_analysis
  [KCL-wiki]: https://en.wikipedia.org/wiki/Kirchhoff%27s_circuit_laws#Kirchhoff's_current_law_(KCL)
  [GaEl-wiki]: https://en.wikipedia.org/wiki/Gaussian_elimination
  [GaEl-wiki-pseudo]: https://en.wikipedia.org/wiki/Gaussian_elimination#Pseudocode
  [rust-result]: https://doc.rust-lang.org/std/result/#result-and-option
