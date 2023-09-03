A Tiny SPICE Simulator
======================

A teeny-weeny SPICE simulator implemented in Rust. It can read a (simple) SPICE
deck, perform the (limited) analyses listed in the `.control` section and write
some waveform data to a file.


Supported SPICE Deck Stuff
--------------------------
Currently supported components:
* `V` - voltage source, `DC` and `SIN()`
* `I` - current source, `DC` and `SIN()`
* `R` - resistor
* `C` - capacitor
* `D` - diode (basic)

Analyses supported:
* `op`    - DC operating point
* `trans` - Transient analysis

In SPICE decks:
* The 'first-line is a title' behaviour is supported
* Engineering notation is supported, e.g. `1k` is 1000
* Node names must be integers
* A control block with a small list of commands between `.control` and `.endc` is
  supported


Unsupported Stuff
-----------------
A _huge_ list of things are _not_ supported. Everything not listed above, which
includes:
* DC Sweeps are not supported
* Sub-circuits are not supported
* MOSFETs and other transistors are not supported
* Noise Analysis is not supported
* Circuit topology checks are not supported
* Even simple commands such as `print` and `plot` are not supported


Running a Simulation
---------------------
The binary name of the teeny-weeny SPICE simulator in this repo is `tiny-spice-rs`.
Since it is written in Rust, so you'll need Cargo and all that to run simulations.

As an example, let's run a transient analysis on a full-wave rectifier. A sinewave
voltage source drives one pair of terminals of the diode bridge. The other pair
of diode bridge terminals has a capacitor and resistor in parallel as a load.

The circuit in `ngspice/fullwave_rectifier.spi` is this:

```spice
Full-Wave Rectifier

V1 1 2 SIN(0 3 1e3) ; input voltage
V2 2 0 0  ; ground, and current measure

* full-wave rectifier
D1 1 3 ; anode (a) cathode (k)
D2 4 1
D3 2 3
D4 4 2

* Load
Rl 3 4 1k

.control
  tran 1us 2ms 
.endc
```

Build the simulator, and run the simulation using the command below:

```bash
cargo run ngspice/fullwave_rectifier.spi
```

Waveforms will be stored in a file called `waves/fullwave_rectifier/tran.dat`.
Waveforms for all nodes in the design will be in this file. The file is in
TSV format (tab-separated values). The first row has the column names. The
second row has the units of eac column. All other rows contain waveform data
in floating-point format.

```TSV
Time	v(0)	v(1)	v(2)	v(3)	v(4)	i(0)	i(1)
s	V	V	V	V	V	A	A
0	0	0	0	0	0	0	0
0.0000005	0	0.004712387042492636	0	0.00235619598234572	
0.0023561910601553058	-0.00000000014040504178809277	0.0000000000000000000005
008837102950005
```

To view the waveforms, load in a spreadsheet and chart some columns. For example, chart:
* the input voltage `v(1)-v(2)`
* the output voltage `v(3)-v(4)`

(If you have `python3` and `matplotlib` installed, try:

```bash
python3 bin/r8n -expr "1-2,3-4" waves/fullwave_rectifier
```

Tools Used
----------------------

Needed:
* Rust 1.71.1

Development stuff:
* KST2 
* Python 3.8.10
 * matplotlib
 * python3-tk


Other Notes
-----------
* Some documentation on how the solver works in `docs/`
* Example SPICE files in `ngspice`
* There are some tests: use `make test`

