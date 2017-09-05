# Development Log

## 2017-09-05
I'm having trouble figuring out how I might handle ODEs in matrices, so the new
plan is:
1
2. Update Guassian Elimination algorithm


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



