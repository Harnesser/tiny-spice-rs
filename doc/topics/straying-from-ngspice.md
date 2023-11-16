# Straying from Ngspice

## Why would You Even Want to?
Because part of the craic of writing your own simulator is that you get to
experiment and play with things that tickle your fancy. I'm not trying to
compete with Ngspice here - that'd be absurd. But i often want to compare
output against Ngspice.

But that basically limits my SPICE parser to match Ngspice's. So i can't
actually experiment with SPICE representations. It also means i can't
experiment with what `.include` or `.lib` means for `tiny-spice-rs` either,
really, if i want to run the same input files in Ngspice too.

## Solution?
What about having a function that can dump out `tiny-spice-rs` circuits in
Ngspice compatible formats? I don't have to worry about writing `.tsr` circuits
that can be read into Ngspice - just dump it out in a compatible format and run
`Ngspice` on that instead?

TSR eventually becomes a transpiler.




