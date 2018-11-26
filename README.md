A Tiny SPICE Simulator
======================

A teeny-weeny SPICE simulator implemented in Rust.


Currently supported components:
* `V` - voltage source, `DC` and `SIN()`
* `I` - current source, `DC` and `SIN()`
* `R` - resistor
* `C` - capacitor
* `D` - diode (basic)


Analyses supported:
* `op`    - DC operating point
* `trans` - Transient analysis

Huge list of things not supported. Everything else, including:
* Sub-circuits
* MOSFETs
* Noise Analysis
* Circuit topology checks



Configuration
----------------------

* OpenOffice 5.0.3.2
* Rust 1.30.0
* KST2 
* Gvim 7.4
* Python 2.7.6



