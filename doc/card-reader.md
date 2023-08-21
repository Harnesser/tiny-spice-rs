# SPICE Card Reader

How SPICE compatible do I want this to be?

## Comments
* First line is always a comment
* Comment lines start with `*`
* Skip blank lines
* If a line contains `;`, ignore that and the rest of the line

## Numbers & Identifiers
* `<value>` is floating-point, or decimal or engineering notation
* `<node>` is a string
* `<ident>` is a string

## Circuit Elements
* `0`, `gnd`, and `GND` are synonyms for the ground reference node
* `R<ident> <node> <node> <value>`
* `I<ident> <node> <node> <value>`
* `V<ident> <node> <node> <value>`
* `D<ident> <node> <node>`

## Control Section
* A list of commands between `.control` and `.endc`
* Set a simulation option: `option <VAR>=<value>`
* Run DC operating point: `op`
* Run a transient simulation: `tran <time> <time> <time>`

## Displaying Simulator Outputs

* Inside the control section
* Print the last computed value:
 - Print all currents and voltages: `print all`

Plot? `v(<node>)` or `v(<node>, <node>)` `i(<node>, <pin>)` `#branch`?
 
Do I bring in a dependency, or just write a recursive descent?

What do I do with `plot` and `print`? And, how do I check them?


### Print

### Plot
I don't want to write a waveform viewer.

So do I just have `keep` instructions? Or, just dump out all waveforms.
Just have a command to control the filename?

I could write data and instructions for KST2 or gkwave. 
But then for the sweeps, I need something else.

### Sweep
Sweeps would be interesting too - could I sweep a resistor value?

# Testing
Select a circuit that can have a interesting DC operating point solution and
that can be used for transient simulation.

In the spirit of "this is not a commercial simulator so I don't have to go
flat out with specs and tests", I think I might do this to test:

## Preparation
1. Make a folder with SPICE decks I want to have working in `tiny-spice`.
2. Make them all have `print all` statements in the command section.
3. Run them in `ngspice`.
4. Copy-paste the `ngspice` `print all` outputs and insert them as comments
   into the SPICE decks
5. Write some Rust code that can extract the expected values from the
   SPICE deck.

Will this fall foul of differences in the simulator accuracies, and differences
in the parameters like `RTOL` and things?

Maybe just implement a checking function?

## Testing
1. Run all the SPICE decks in `tiny-spice`
5. Write a tool to extract expected values from the comments in 


