# Development Log

    This is a personal project. It does not have to be rigourously tested.

## 2023-11-08 VCCS

### Fixing `G`
After a lot of pondering, fixed a bug with `G` at least. The `k` factor needs
to be part of the system of equations that the Gaussian Elimination step solves
not, as i had it, calculated as part of an `evaluate()` step with the current
and previous node voltages.

The `E`s are not fixed yet, but the drum machine circuit uses an opamp macromodel
with `G`s anyway, and I've seen something that looks half-working.

![Kickdrum waveforms](./images/tiny-spice__kickdrum_basic.png?raw=true)

### Testing
There is also some basic dc op self-checking sims. The idea i kinda got from 
Tsoding's `nob` streams is to basically include the top half of the main
`tiny-spice.rs` program to read in a SPICE file and elaborate the circuit. Then
the analysis routines can be called programmatically and asserts can be run on
some results. Before this point, there was no value checks on analyses launched
from SPICE decks.

The test SPICE files have sinewave sources and transient analysis set up in the
`.control` blocks. The unittests instead just do a DC operating point analysis
on the circuit, hacking an offset in the sinewave sources to get a non-zero DC
value.

### Fixing `E`
VCVSs are fixed now too. Like the independent souces, a new row+col is added in
the matrix for the output of the VCVS. There are +1 and -1 in the node rows to
add the source branch current to the equations. The src row links the input 
control voltage to the output control voltage by the `k` factor.

## 2023-10-21 Opamps Don't Work
The opamp cct i made with a `G` and an `R` doesn't work. The same cct works in
ngspice.

I did notice the other day when looking at the data for the nodes that the 
VCVS took a 'cycle' to kick in. Maybe this is the problem? I should be using 
the current voltage values in the solver rather than the last time point? Nah,
that's stupid. At the very first solve, that matrix would be 0 anyway.

Changing the commmand to an `op` and switching out the sinewave voltage 
source to a 1V dc source - still results in a failure. So there's something
fundamentally wrong in the engine that can't solve this.

## 2023-10-19 PWL, E & G
Recently, i'm going to implement stuff that supports studying electronic drum
machine circuits. To that end, I've
* V PWL for the gate pulses
* VCVS (`E`) and VCCS (`G`) because I want to model an opamp

After I had implemented the instancing thing for `R`s, `C`s and `D`s,
implementing `E` and `G` was fairly straightforward. The only tricky bit was
registering the "output" voltage source for the VCVS.

It might be nice to implement `.param` for the toplevel. Also, I feel
`.include` needs to happen soon?

Version v0.8.0 is on github, but i had to do a few bugfixes afterwards.

## 2023-09-28 Subcircuits with Parameters
This seems to work now, and even bracket expressions with a single identifier
are supported in the parameter overrides.

The circuit `param_fullwave_rectifier.spi` instantiates 3 versions of a 
bridge rectifier subsystem. The capacitor value in the `rc_load` subcircuit
is parameterised, passed down from the instantiation of the subsystem at the
toplevel.

I'm quite pleased with this.

## 2023-09-23 Parameters - Numerical Literals
Refactored the `multi_` subcircuit example to have 3 instantiations of the
`system`. A system is:
* A diode bridge subckt.
* A series resistor, just to get a new node in the system
* A resistor load `r_load` where the resistance is split up, again to
  get more nodes in the system.

Each system has a different cap load. I'm running the sim to 5 ms which isn't
that fast, tbh. A profiling job later?

Can I get numeric literals for the primitives working again?

## 2023-09-23 Parameters for Primitives
Decided to keep primitive instantiations as actual instantiations until the
circuit expansion phase. This means I can keep expressions in the instantiations,
but the primitives themselves just have a value. I won't have to touch the
engine code to do resolve bracket expressions or anything - this is all done
when decending the hierarchy.

Faoi laithar, the device paramters get default values. Next steps:
1. Parse and apply parameters that are numeric literals
2. Propagate and lookup single parameter bracket expressions in the form,
   e.g. `{rval}`.

## 2023-09-19 Parameters
Parameters would be a nice addition to subcircuits. Maybe just allow 1 term
in the expressions?

## 2023-09-18 Recursive Subcircuits
I think I've recursive subckts working. I needed to add node aliases for all
subckt ports as I'm decending, so if I'm connecting to a port above, I can get
the `NodeId`.

Need to add support for all devices, and tidy up tests and clippy, and I think
that's 0.8.0

## 2023-09-17 Subcircuits
Node names don't have to be integers any more.

There is some limited support for subcircuits now. The simulator can handle 1 level
deep of subcircuits, but only for resistors, capacitors and diodes. There's a
version of the fullwave rectifier that has both the RC load in a subcircuit and the
diode bridge in another, and it simulates like flattened one!

The R, C and Ds now have an identifier now. The other circuit elements do not. This
caused lots of testmode reflow, and I'm not sure if they really need identifiers...

I'm surprised the waveform dumping still works though - I thought the matrix would
be a mix of Vs and Node voltages now, and not partitioned off nicely.

The subckt code is _very_ copy-pasty. It also doesn't "cache" subcircuits it'll
build everything from scratch each time its instantiated. Good enough for this
exercise. The code is in `spice::Reader::circuit()`. I dunno if this is a great
place for it.

For a 0.8.0 release, I'd want to:
* allow all currently-supported circuit elements
* allow more than one level of hierarchy

## 2023-09-09 Part 2
Nevermind, I was looking at the wrong columns of the waveform data.

## 2023-09-09
The fact that my simulator can give the wrong answer for the diode bridge is a bit
of a disappointment. Whatever about time-step-too-small, I'd much prefer that to
giving the wrong answer.

How do I attack this? Well, first of all, I don't really have any good checks on
transient simulations. The fullwave runs 2 cycles of sinewaves. I'd like to have
spot-checks on the output voltage on each peak of the input (maybe the zero
crossings too?). To do this, either a implement a `measure` command, or inspect
the output waveform data somehow.

### Implement `measure`?
Say `measure` was implemented. Would `if` have to be implemented too so that the
control block could be run in NGSPICE too? Can a measurement be checked (asserted)
in NGSPICE somehow?

Maybe this spot-checking has to be done on "constructed" simulation runs rather than
SPICE-card runs? Is there a `continue to` command in NGSPICE? Can a transient
simulation be run for longer?

Should the main executable be changed to return different status values an
assert fails? Otherwise, we'd need to go grepping the logfile?

### Interrogate Waveform Data
Two options for this:
* Post-processing: read the waveform data and do stuff
* Investigate the data by peering into the analysis object at the end of a run


## 2023-09-02
I decided to not support `print` or `plot` yet (if ever) and to just dump waveform
data. I'm not writing a waveform viewer too.

I updated `r8n` to do simple plots from the waveform data.

The fullwave rectifier doesn't simulate well. I thought I'd done a big loop of sims
to test such a thing. It works in `ngspice`, but interestingly not if the cap load
is > ~500 nF! So I've a correctness issue and a convergence issue, possibly. It
may need some tweaks to `RELTOL` and the like. And for that - do I need to support
allowing these to change in the `.control` blocks?

Prepping for a release of 0.7.0.

## 2023-08-21
Still trying to figure out how much SPICE to support.

I fixed voltage sinewave sources, I think, and gave some test unique names. Having
multipe tests named `test` is not great for the summary.

All voltage sources know their index past the node-count. This is so we can
stamp them without lookups. That said, we still look up the index of the "known"
column to place the voltage source values in.

I downloaded KST2 again. I can't find anything better for waveforms atm.


## 2022-10-15
What if the next step is to just make everything I have working now be doable from a
SPICE card? That might make a nice next release.

Then, I have 2 problems:
1. Check that the reader works, parsing-wise
2. Check that they trigger the correct analysis

My imagination for this is, uncommonly, too much for this. I need to reign in things and
just do something that I can cut a release for. I need to write some requirements. Some
simple ones. Do I want to write an EBNF?

## After I get through this
In general though, I want to understand noise better. So after this, I want to see
how to implement noise analysis and do that thing where you can get the noise contributors.

I think after this I want to tackle subcircuits too. Noise or subcircuits?

### Testing
I have 5 SPICE tests running at the moment.
* 2 of them are unittests of `spice.rs` checking the parsing of numbers
* 3 are analysis tests that read in SPICE files

I have a problem with testing the results of the analysis tests. Do I need golden waveforms
to check against in the case of the transient tests? For the DC tests, I have comments in
the file with the node voltages that `ngspice` produces. Do I need to parse and check these
against `tiny-spice` outputs?

But none of my transient tests are self-checked at the minute, I'm only testing if they
don't raise a "timestep too small".

### VSIN Initial Value
How do I, say, set an initial value for a sinewave node? Can I do this? How would I get
a `dc` value for a voltage source to be the starting point for a `VSIN`, for example?A

Maybe there's some kind of check that the `dc` value is a valid solution to A sin(wt) and
set the phase accordingly? Is this even how SPICE does things or are `dc` and `trans`
totally separate - they can't be, right?

There is an offset in the VSIN definition, do I do the right thing here for `op` analysis?
This is where it would be nice to do a sweep of a source parameter. Is that offset a DC
offset or is it the initial value?

### Plots
I also want an command that I can just run and have waveforms pop up afterwards. Then I'd
have to deal with batch mode and immediate mode runs of the simulator.

Maybe `plot` for this means writing out a waveform data file? What if there are multiple plots?
If I was targeting KST2, I could write the data file that is the sum of all the waveforms,
and also write out KST2 commands to plot the overlays.

What should this do in `tiny-spice`?
```
    plot v(1)
    plot v(2), v(3)
```

What would I want it to do? If not in batch mode, pop up some windows with plots. If in batch
mode, dump some pngs somewhere. But this is all extra shit. I don't want to have to write a
GUI for waveform display, but maybe, in the real world, this just /IS/ part of a simulator.

So maybe I don't implement `plot`, but instead implement whatever the SPICE equivalent of
`keep` is? And this infers a separate database for waveforms, I suppose.

So this then is how many of the waveforms should be stored off as the simulation progresses?
If you can do plots after the fact, then everything has to get stored in the waveform database.
And now there's a new object - the "waveform database".

### `print all`
This is the easiest thing to implement, right?

### Quit
The `quit` command assumes there's an interactive mode. I do not have an interactive mode
in `tiny-spice`.


## 2018-11-26
Between upgrading my desktop, getting a new laptop, losing and finding my
github stick, I needed to consolidate my github repos.


## 2018-02-02
Can't remember where I was and what's left to do.

Ok, now I remember - I'm trying to write a spice deck reader. I'm not sure how far
I got, and what I'd consider a minimum viable product.

Suppose:
1. I, V, R, C, D
2. `trans` and `op` in `.control` blocks
3. Test all this
4. Waveforms? Do I need a `--output` switch, maybe?

Getting weird results from the command line testing:

    test spice_irrc ... ok
    test spice_irrrr ...   [ELEMENT] Current source: 0.015709353792572548A into node 0 and out of node 1
    test spice_reader ...

It looks like `.success()` is not reliable.

How do you test a SPICE engine?


## 2017-11-23
Rustup

https://users.rust-lang.org/t/how-do-you-test-binaries-not-libraries/9554/9


## 2017-11-17
Can read SPICE files now (crudely). The simulator engine now uses a configuration
object to store parameters like TSTEP, RELTOL and waveform filenames. All tests
have been updated to use this new scheme.

Todo:
* find out how to test binaries

Decisions:
* not going to support multiple analyses in .control block


## 2017-11-07
Started `tiny-spice.rs` which is the toplevel binary to tie everything
together. I'm trying to write this and the SPICE file reader at the
same time so I can figure out what the interface should be.

## 2017-11-06
Implemented thing to read values like 1 or 1.0 or 1.0u.

## 2017-11-05
Initial musings on a SPICE deck reader.


## 2017-11-04
Adding a small cap across the diodes seems to make everything happy.

Fixed up a bunch of testcases and stamped version [TAG 0.6.0]

## 2017-11-03
Created a program to plot all the results from the loops I've got going.
It's called `r8n` and it plops a dot down at the last point of the waveform
that was calculated - showing me easily what broke.

Developed this to test out the SPICE engine as I have it now. I'm getting
failures on the diode-bridge loop that has an RC load. There's no obvious
correlation between the failing testcase and cap, or timestep or anything.


## 2017-11-02
Threw a Hail Mary and changed `f32` to `f64` throughout the program. This
seems to fix things!

What I'd like to do before stamping the next release:
 1. Look at waveforms generated by `fullbridge_loop`
 2. Make all sims pass
 3. Try reverting diode model back to a previous version to see
   how simple it can be and still converge.

What do I want to do with this in the future?
 * SPICE card deck reader
 * Circuit topology checks - no DC path to ground, unconnected nodes etc.
 * Try out some different integration methods
 * MOSFET model
 * Hook up to my tiny-verilog simulator to make a mixed-mode thing
 * Linear, NonLinear, NonLinearDiff partitions
 * Sub-circuits

What's this for, though? Why am I doing all this again?


## 2017-11-01
Downloaded SPICE2 FORTRAN code and coded up the diode voltage limiting
algorithm to match.

Still failing transient sims.


## 2017-10-31
More messing with the Colon/Nagel thing. Use `Cell` to remember the previous
values of things without making the entire `Diode` structure mutable and
infecting the simulation engine.


## 2017-10-24
Some light internet searching. Maybe I have to try GMIN-stepping and
source-stepping algorithms?

Could I ask someone at work? One of the Daves? Marie?


## 2017-10-23
Colon didn't help. The V_crit is around the knee of the diode, so doesn't help
with the cycles too much.


## 2017-10-22
Going to try that Colon thing I found in Nagel. For this though, I need to know
the previous value of the voltage across the diode. Should I just add this to the
structure or what?


## 2017-10-21
Found Nagel's SPICE2 paper.


## 2017-10-14
I spent the last month or so trying to come up with a fix for the diode model
transient analysis problem. There was a bug in how I calculated G_Eq for the
diode companion model. I've made some progress, but the diode model doesn't
seem robust yet in transient analysis.

In the course of this investigation, the engine has been updated to print more
information about which RETOL etc it's using, and it spits out messages when it
changes the timestep. I also make all the analyses return a result datastructure
so that the unit-tests can determine if the circuit time-stepped-to-small or not,
which is useful for robustness testing.


The circuit I used during the investigation was:
* Current source and resistor - 20V
* Diode bridge with 2 diodes commented out
* 1k load resistor

The diodes are in series with the resistor, one before the R, and one after. This
turns out to be a harder circuit for the simulator to solve than the full diode
bridge! The solver seems to be getting into a limit-cycle when solving for the
case where the input falls and crosses 0V and the diodes start to go into reverse
bias.

At least I think the simulator finds the 2-diode circuit more difficult to deal
with. What I really need to see is a full test suite - same circuit but varying
all the VNTOL, RETOL, saturation current of the diodes, etc.

I also need a SPICE file reader. And I need to decide when I'm finished.

I'm tempted to write the SPICE file reader next, as this will help with gathering
together circuits for the robustness testing. But this isn't trivial - I'll need
to write a parser, I'll need symbol tables, and I'll probably be tempted to expand
to control blocks and option blocks.

For robustness testing, it's this or writing a bunch of testcases in Rust which'll
all write to the same log file and stuff. I need to recompile every time there's
a circuit change.

## 2017-09-16
Had a go at simulating capacitors in transient.

With an RC low-pass filter test circuit, I'm seeing a few problems:
1. The output wave seems to depend on timestep, not on input wave
  frequency.
2. The output wave is leading the input wave.

[FIX] - wire up the current source in the capacitor companion model in the
correct direction.

And I've noticed on the diode bridge sim:
1. It's not very robust with timestep.


## 2017-09-15
Got transient analysis working! I even tried it on the diode bridge
circuit and it was a success!

Interestingly, even with nonlinear devices in there, the timestep maxes out on
this. Maybe that's to be expected - I've nothing that holds energy in the circuit
so I don't need integration and all that.

The logfile produced for the diode-bridge transient simulation is 4.5M. This is
huge.

Where to next?
* L & C models
* Write out engine metrics
* Propper logging to quieten output
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
My moves are nonlinear, but that's ok.
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

Next: figure out how nonlinear devices live in an enumeration for linear
devices.


## 2017-09-06
Turns out that ODEs are not stored in the matrix:
1. The nonlinear circuit elements are linearised around an operating point
   using Newton-Raphson.
2. Values are used to "stamp" the matrix with a linear companion model of the
   nonlinear element
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
3. Start on nonlinear solver
4. Remove the [0] column and row from the matrix to save space. This means either
   changing the node index of GND to something other than 0, or littering the code
   with lots of `[i-1]`s.
5. Do LC transient analysis.

The nonlinear solver is the most interesting bit, I think. Although LC transient
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
 d. Newton-Raphson for nonlinear equations

2. Transient Simulation
 a.



