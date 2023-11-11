## Libraries

I'd like to be able to include circuits from multiple SPICE decks to make one
entire circuit. I'd also still want to be able to run the subcircuit SPICE files
individually and pick up the default `.control` thing.

### My Idea
So, my idea would be to have a command to include another SPICE file into the
current design, but to import only `.subckt` definitions.

This turns out to be basically ngspice's `.lib`, but i have to modify the files
i'm importing to have `.lib` and `.endl` sections. And I have to search for the
specific library name.

### `Ngspice`

Ngspice has `.include` and `.lib`. They don't really say what the difference is,
but you have to call `.lib <filename> <libname>` and it seems you have to have
things between `.lib` and `.endl`. This would mean i'd have to patch existing
files...

### What should happen?

So there's two things:

    .lib <filename> <libname>             --> import
    .lib <libname> <SPICE STUFF> .endl    --> definition

The program needs to read the main SPICE deck until it hits a `.lib` statement.
It should expect 2 strings after the `.lib` statement: a `filename` and a
`libname`.

Next, open `filename`. If it's not found, fatal error. 

Search paths? From the root dir of the current file, and from the root dir of
where the simulator was launched? Ngspice seems to do this.

It should run the SPICE deck reader on the file again. This time ignore
everything until a `.lib` statement is found at the start of a line. If no
`.lib` statement is found, fatal error.

If a `.lib` statement is found, there should be exactly 1 string argument, the
library name `libname_scan`.  If `libname_scan` does not match the requested
`libname`.  Keep going?

If `libname_scan` is the one we're looking for, parse everything in it up until
`.endl`.  If we reach the end of the file without an `.endl` - fatal error.

Should i ignore other toplevel statements in `.lib` definition sections? Should
I only pick out `.subckt`s (and later `.model`s)? Ngspice `.includes`
everything between `.lib` and `.endl`.

What if the opamps were in their own lib, and i needed opamps in the different
subckts in libraries? Keep the `.lib` imports outside the `.lib` definitions,
import them at the toplevel? This is a downside of the "partial include"
version of the `.lib` statement. But i can see how "just get something working"
happened.

### So, `.include` too?
I may as well implement `.include` too as `.lib` is a special case of this.

So my `read_spice()` function should take an argument: Full, lib. And it will
have to be called recursively too.

### Architecture
Currently SPICE decks are read by the `read()` function on the `Reader` struct.
The reader struct has an array of circuits. The `read()` function updates these
things.

How many times do I call `read()` in tests and stuff? Only a handful of times in
the test section. So keep this interface, and have a subfunction that can do
everything.

Interestingly, the `Reader` struct has an index to the current circuit that its
building. This means an `.include` inside a `.subckt` should do the right thing
by accident?



