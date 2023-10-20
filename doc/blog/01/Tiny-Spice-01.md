# A Teeny Weeny SPICE Circuit Simulator

Ionsamhlóir Ciorcaid An-bheag.

## What and why?
I wrote a small SPICE circuit simulator to get over my fears of `RELTOL`,
`ABSTOL` and time-step-too-small errors.

I'm at version `v0.8.0` which has quite a nice set of basic features. It can read
SPICE decks with circuit descriptions. It can execute some commands if they are
listed in a `.control` block in the SPICE deck. It can do 2 types of analyses:
DC Operating point; and Transient. Circuit device-wise, it can imagine resistors,
capacitors and diodes. Sources supported are voltage and current sources
(DC or sinewave).

It's written in Rust, cos that's what I like to use instead of C when I can.
https://www.github.com/harnesser/tiny-spice-rs. See the README for details of how
to simulate a circuit.

## Subcircuits!

One of the things I'm most happy about is that it supports subcircuits! And
the subcircuits can be parameterised! And parameters can be very simple one-identifier
expressions! 

My working example is 3 copies of a fullwave rectifier system with parameterised loads.
The SPICE for this circuit is shown below, as is a cartoon of the circuit.

![Circuit](./tinyspice_param_fullwave_rectifier.png?raw=True)

ALT-TEXT: Circuit diagram showing 3 instances of a subcircuit. The supply to all three
is a stack of sinewave sources at different frequencies and amplitudes. The subcircuits
themselves are subcircuits: a diode bridge rectifier, a series resistor and an RC load
with a parameterised capacitor value. The capacitor value is passed down to the capacitor
in the RC load subcircuit all the way from the toplevel instantiations.


These waveforms are the proof that it works.

![Waveforms](tinyspice_subckt_params.png?raw=True)

ALT-TEXT:
Waveforms from a transient simulation of the above 3-bridge circuit. The input 3-tone
sinewave is shown, as are the voltages across the 3 RC load blocks. The different 
parameterised values for the three blocks result in different smoothing curves.


## Where next?

Next, maybe something with 
[reciprocity|https://en.wikipedia.org/wiki/Reciprocity_(electrical_networks)],
that seems interesting. I _think_ that reciprocity can be used in noise simulations
to work out the contributors to noise at a certain node.

A simple waveform viewer would be nice, but I've no intention of writing one of those.
Even though there's basic `.control` support, I don't do anything with `print` or
`plot` commands.


