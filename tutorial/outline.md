Outline
=========================================

Writing a SPICE simulator in Rust.

Should I do a parallel implementation in Python, just so the blog series
is easily portable between programming languages?

Prolog
-----------------------------------------

### 00: What is this all about.
Write a SPICE simulator in Rust. Minimal - just to get a feel for what
a real simulator does, and the parameters that it uses.

Analyses:
* DC operating point
* DC Sweep 
* Transient

Components:
* R, V, I
* C, L
* D
* MOSFET

In the style of other follow-along-at-home blog post series, we'll start
with a minimum DC operating point solver with current sources and resistors
and build it out from there.

Assumes knowledge of how to use SPICE simulators, and to be somewhat at
ease with programming in general.

This is about putting a SPICE simulator together - it's not a tutorial on
Rust, or on the theory of SPICE simulators. I may touch on the basics, but
there will be copious links to other resources on the web that are more
tutorials on these things, or in-depth information.

But if you're interested in one aspect of this, and have nearly no knowledge
on the other, I've faith that you're smart enough to fill in the blanks quickly.


### 01: Rust
* Small speil about Rust

Q: Is there any small piece of the simulator we can write now
 just to get going?


### 02: Anatomy of a SPICE Simulator
* Small speil about SPICE
* General linear, nonlinear, analysis


Time 0
-----------------------------------------

### 00: Defining a circuit
* MNA - resistors and current sources
* MNA extended - voltage sources
* Building the matrix from the netlist - stamping

### 00: DC operating point
* solve - guassian elimination


Sweeps
-----------------------------------------

### 00: DC Sweeps
* Sweeping a voltage source
* This gets us some graphs - how to display these?

### 00: Transient analysis
* need interesting sources - SIN()
* timesteps
* simple sinewave over, say resistor divider - not too interesting or
  different from a DC sweep at this stage, but lays some groundwork 
  for the following sections


Energy-Storing Elements
----------------------------------------
Where transient analysis gets interesting.

### 00: Capacitors
* linearisation about a point
* convergence - VNTOL, RELTOL, ABSTOL
* integration methods - in fact, I haven't explored this much

### 00: Inductors
* should be an easy addition after getting capacitors working?


Diodes
---------------------------------------

### 00: Nonlinear Differential
* The diode
* Newton's method, linearisation about a point


Tiny-Spice
---------------------------------------

### 00: Program Organisation
* launching analysis
* configuration object with RELTOL and stuff
* Return results
* datastructures for MNA 


### 00: Parsing SPICE netlists
* comment line
* elements
* options to set RELTOL etc
* control blocks, or `.dc`?
* command line stuff
* writing waveform files



Extra Credit
----------------------------------------
These would be stretch goals for the tutorial.

### 00: FFI a proper MOSFET model?
Interesting to do - how would I take a BSIM3, for example, and integrate
it into this simulator?

This would be a real differentiator for this tutorial. It would also be 
a good goal of the simulator to be able to run the benchmark test spice
files that are included with the BSIM4 model.

It would also mean the simulator would have to be able to parse model cards
for mosfets and stuff, and apply them to the model.


### 00: WASM to run spice in browser?
If I've a javascript waveform viewer, then this might be nice too.
Everybody loves WASM.
Atwood's law.

### 00: Sub-Circuits
Do something with subckts?
At this stage, any tweaks ripple through all aspects of the simulator:
parser, solver, waveform writer.

