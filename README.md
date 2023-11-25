A Tiny SPICE Simulator
======================

Ionsamhlóir ciorcaid an-bheag.

A teeny-weeny SPICE circuit simulator implemented in Rust. It can read a (simple)
SPICE deck, perform the (limited) analyses listed in the `.control` section and
write some waveform data to a file.

Supported SPICE Deck Stuff
--------------------------
Currently supported components (alphabetically):
* `C` - capacitor
* `D` - diode (basic)
* `E` - voltage-controlled voltage source (VCVS)
* `G` - voltage-controlled current source (VCCS)
* `I` - current source, `DC` and `SIN()`
* `R` - resistor
* `V` - voltage source, `DC`, `SIN()` and `PWL()`
* `X` - subcircuits

Analyses supported:
* `op`    - DC operating point
* `trans` - Transient analysis

In SPICE decks:
* The 'first-line is a title' behaviour is supported
* Engineering notation is supported, e.g. `1k` is 1000
* A control block with a small list of commands between `.control` and `.endc` is
  supported
* Limited bracket expressions are supported for subcircuits, `R` & `C`.

Unsupported Stuff
-----------------
A _huge_ list of things are _not_ supported. Everything not listed above, which
includes:
* DC Sweeps are not supported
* MOSFETs and other transistors are not supported
* Noise Analysis is not supported
* Circuit topology checks are not supported
* Even simple commands such as `print` and `plot` are not supported

Running a Simulation
---------------------
The binary name of the teeny-weeny SPICE simulator in this repo is `tiny-spice-rs`.
Since it is written in Rust, so you'll need Cargo and all that to run simulations.

As an example, let's run a transient analysis on a full-wave rectifier circuit.
A stack of sinewave sources drives the inputs of three subcircuits. Each of the 
subcircuits has a full-wave bridge rectifier circuit and an RC load with a
parameterisable capacitor value. The circuit is drawn below:

![Parameterised fullwave recitifiers](./doc/blog/01/tinyspice_param_fullwave_rectifier.png?raw=true)

The circuit in `ngspice/param_fullwave_rectifier.spi` is this:

```spice
Full-Wave Rectifier with parameterised subcircuits

* 3 instances of a diode bridge + RC load
* cap load in each instances parameterised and overriden from
*   the toplevel

V1 vstack1 gnd     SIN(0 5 1e3) ; input voltage
V2 vstack2 vstack1 SIN(0 2 2e3)
V3 vstack2 IN_p    SIN(0 1 3e3) ; flip to differentiate between "multi_"

* full-wave rectifier
.subckt bridge bp bn ba bb

  D1 bp ba
  D2 bb bp
  D3 bn ba
  D4 bb bn

  * Small caps across the diodes to prevent time-step-too-small
  CD1 bp ba 12pF
  CD2 bb bp 12pF
  CD3 bn ba 12pF
  CD4 bb bn 12pF

.ends

.subckt system sinp sinn soutp soutn cval=10uF
  Xbridge sinp sinn midnode soutn bridge
  Rd midnode soutp 1
  Xload soutp soutn rc_load cvalo={cval}
.ends

* Load
.subckt rc_load in1 in2 cvalo=1nF
* Split R so we have internal nodes
  Rl1 in1 la 200
  Rl2 la lb 300
  Rl3 lb lc 400
  Rl4 lc in2 100
  Cload in1 in2 {cvalo}
.ends

Xsystem1 IN_p gnd vp1 vn1 system cval=1uF
Xsystem2 IN_p gnd vp2 vn2 system ; DEFAULT cval=10uF
Xsystem3 IN_p gnd vp3 vn3 system cval=100uF

.control
*  option reltol = 0.001
*  option abstol = 1e-12

  tran 100ns 5ms
  option ; ngspice only shows new values after analysis

  plot v(IN_p) v(vp1,vn1) v(vp2,vn2) v(vp3,vn3); (ngspice)
.endc
```

Build the simulator, and run the simulation using the command below:

```bash
cargo run ngspice/param_fullwave_rectifier.spi
```

Waveforms will be stored in a file called `waves/ngspice/fullwave_rectifier.spi/tran.dat`.
Waveforms for all nodes in the design will be in this file. The file is in
TSV format (tab-separated values). The first row has the column names. The
second row has the units of eac column. All other rows contain waveform data
in floating-point format.

To view the waveforms, load in a spreadsheet and chart some columns.

(If you have `python3` and `matplotlib` installed, try:

```bash
python3 bin/r8n -expr "4, 5-6, 7-8, 9-10" waves/ngspice/param_fullwave_rectifier.spi
```
)

![Fullwave rectifier waveforms from tiny-spice-rs](./doc/blog/01/tinyspice_subckt_params.png?raw=true)


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

