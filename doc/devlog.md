# Development Log

## 2017-09-07
I hope to implement DC operating points with diodes in the circuit.

Things I have to solve:
* Generating parameters for a diode companion model
* Stamping some kind of base linear matrix with diode companion model
* Updating the matrix after every iteration. Either
 * keep a linear matrix base around and reuse
 * stamp and unstamp
* Convergence testing: VNTOL, ABSTOL & RELTOL


## 2017-09-06
Turns out that ODEs are not stored in the matrix:
1. The non-linear circuit elements are linearised around an operating point 
   using Newton-Raphson.
2. Values are used to "stamp" the matrix with a linear companion model of the
   non-linear element
3. Run Gaussian Elimination on this matrix to compute the unknowns
4. Look for convergence, where v(n+1) ~= v(n) and stop.

Companion model for a diode is a current source in parallel with a resistor.
And possibly a GMIN resistor too.


## 2017-09-05
I'm having trouble figuring out how I might handle ODEs in matrices, so the new
plan is:
1. Update circuit builder to MNA
2. Update Guassian Elimination algorithm

This actually was done. 

The next step is to read source code of open-source circuit simulators to see
how ODEs are handled.


## 2017-09-02
I've been using PDF on the internet that got the F'(V) for a diode wrong. After
fixing this, the DC operating point algorithm I have converges if the initial 
guess for the diode voltage is larger than what it should be. For lower initial
guesses, things fail.



## 2017-08-31

I can go a few ways now:
1. Update the Gaussian Elimination algorithm to match the better one on wikipedia
2. Update the circuit matrix builder to MNA
  - This will help handle V sources without circuit transformations
3. Start on non-linear solver
4. Remove the [0] column and row from the matrix to save space. This means either
   changing the node index of GND to something other than 0, or littering the code
   with lots of `[i-1]`s.
5. Do LC transient analysis.

The non-linear solver is the most interesting bit, I think. Although LC transient
is kinda interesting too...

Fixed back-substitution.


## 2017-08-30
Have a basic KCL solver for Is and Rs.

## 2017-08-29
What's the core of a SPICE engine?

Well, I need:

1. DC Operating Point
 a. Netlist representation
 b. Node equation builder
 c. Gaussian reduction algorithm
 d. Newton-Raphson for non-linear equations

2. Transient Simulation
 a. 



