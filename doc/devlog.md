# Development Log

## 2017-09-16
Last night I got transient analysis working! I even tried it on the diode bridge
circuit and it was a success!

Interestingly, even with non-linear devices in there, the timestep maxes out on
this. Maybe that's to be expected - I've nothing that holds energy in the circuit
so I don't need integration and all that.

The logfile produced for the diode-bridge transient simulation is 4.5M. This is
huge.

Where to next?
* L & C models
* Write out engine metrics
* Propper logging to quiten output
* MOSFET model?
* The LTE timestep thing?


## 2017-09-14
Think I've an implementation of the iteration-count time-marching loop. Albeit
that I've just ran it on a static circuit, but at least I can see the timesteps
increasing in time because there's nothing happening.

## 2017-09-12
Found an algorithm for time-stepping that uses only iteration counts and
no fancy error calculations (used in SPICE2). See openoffice doc for more 
details.

## 2017-09-10
The unloaded diode bridge does find a DC solution, but if I load the output
with a resistor, there are NaNs all around the place.

It was having problems solving the current through the 0V source to ground.
I put in a hack to make the result 0.0 if the results isn't a finite number,
and things seem to work! [TAG 0.4.0]

### Next?
Where to I go next?

* DC Sweep
 - sweeping parameters
 - recording results
* Transient Analysis
 - [DONE] sinewave source
 - [DONE] sweeping time
 - [DONE] recording waveforms
 - L & C models mean numerical integration routines


## 2017-09-09
Got a simple diode-Isource-resistor circuit to converge by limiting
the voltage I calculate currents for in the diode model. I was quite happy
about this.

But then I tried to go for broke and find the DC solution for a diode bridge.
This did not work.

Liverpool got blown out today. Feck.


## 2017-09-08
I'm gonna stamp some diodes tonight.
Gonna stamp some diodes.
My moves are non-linear, but that's ok.
Gonna stamp some diodes.

The other option is to stamp everything once. And update the parameters at each
iteration. This means keeping the twiddle values and their locations, but means
we don't have to copy/reallocate the huge matrix all the time. After all, the
companion models don't change.


## 2017-09-07
I hope to implement DC operating points with diodes in the circuit.

Things I have to solve:
* Generating parameters for a diode companion model
* Stamping some kind of base linear matrix with diode companion model
* [DONE] Updating the matrix after every iteration. Either
 * [DONE] keep a linear matrix base around and reuse
 * [NO] stamp and unstamp
* [DONE] Convergence testing: VNTOL, ABSTOL & RELTOL

Next: figure out how non-linear devices live in an enumeration for linear
devices.


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



