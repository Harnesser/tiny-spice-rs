# Writing WAV Files

## Goal
I'd want to add a command called `wav` that takes a node name and writes the
data to a WAV file that i can then play over the speakers. It will have to be
called after a transient analysis. Say, 16ksps on both stereo channels. The
samples will have to come from a grid.

The simulations are going to be long here, I'll need a few seconds of info or
so. The sim i have going at the minute doesn't ring for that long, so i'll need
to also experiment with component values, so am i going to need a looping
structure too? Or be able to change parameter values from the command line, and
support `.param` statements at the toplevel?

## Why?
It would be interesting to write a WAV encoder, but maybe i just call out to 
Python or something and have it do it? Yeah, this is a circuit simulator, not 
an audio thing!

I still need to put waveform samples on a grid though...

Y'know what, why not?

## WAV format
A nice diagram of the WAV file format can be found [here][1].

## WAV Writer
Unfortunately i already have a module called `wavewriter` that dumps waveforms.
I should either rename this to something like `waveformdb` or `wavedb` or
something. Then, in the new `wavwriter`, it would take the datastructure from
the `wavedb` and write out the selected waveform in both channels a stereo WAV
file.

I think it could be implemented as an array of `u8`. Then i can push the bits
of data in whatever big or little endian format i need, then write the vector
out to a file i think.

There could be a test module in the new `wavewriter` (`riffwavwriter`?
`wavwriter`?) that generates a sinewave and dumps this out so i can test the
implementation inependent of running a simulation.

## WAV Parameters
Things like sample rate - how should i set them? Should i try to work back from
the `tran` settings and barf if there's an unsupported timestep? This is where
quatntized time storage might help, but i don't know what that'll do for
convergence.

## Samples on the Grid
This means i have to take more care with my `tran` function. I need to ensure
that there's a solve on 0 ns then at each `tstep`, nomatter what timestepping
the engine is doing. It also means only dumping these timepoints to the
waveform database. And it means i need to be able to look up a node name and
find it's index in the waveform database.

So i might as well update the `print` command too to write out a command for
`r8n` at least, since i can look up node names.

When i do this, i want to keep an option that writes out all the timepoints for
debug purposes.

What is `tran` again?

    tran tstep tstop < tstart < tmax > >

Here, `tmax` sets the maximum computing interval, and `tstep` is the grid for
`print` or `plot`. Does `tmax` overwrite `RMAX`?

## Notion - Time Quantisation
Putting samples on the grid brings me back to another thought i'd be having: do
i need to quantise the time grid? Verilog does this, and my notion to eventually
write a "verilog"/"spice" cosimulator would benefit from this. So i'd be
storing the current timestep in picoseconds or femtoseconds or something, and
converting to a `f64` in `evaluate()` methods, gor converting to `f64` before I
do all the calls.

If time is on a grid, then the simulator would have to reject any `tstep`
settings that are not on that grid. What happens if, say `23.4us`? What does
verilog do i you set a delay (in a loop) to `1.1 ns` with `timescale 1ns /
1ns`? I think it silently snaps it to the precision.

If time is quantised, then i'll need a function that'll convert a floating
point number to the timebase.

## Notion - `typedef` on Node Value Type
Another notion i've been having: use a `typedef` for the node value field. I
might want to explore again why things didn't converge in `f32`.


## Notions
These notions smell like things that should be done on a branch.

## Links
* [WAV Format Diagram][1]

[1]: http://hummer.stanford.edu/sig/doc/classes/SoundHeader/WaveFormat/]
