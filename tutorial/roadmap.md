Roadmap
===============================================

Write a SPICE simulator computer program in the Rust programming language.

The purpose of this series of blog posts is to explore how SPICE simulators
work - the analyses, 

It's far from a definitive implementation of a simulator - it's more of a 
learning and exploration exercise. The author originally went on this so no fear
of the likes of `ABSTOL` and `RELTOL` and to get a feel for the algorithms,
especially the integration ones.


We'll start off with circuit representations in code. Then, the easiest first
step is a DC solver with resistors and current sources. 

This will get us up and running with a datastructure to describe an electrical
circuit, and introduce reduction for solving a system of linear equations.

Everything starts with a DC solution.
Might implement one or two simple topology checks.

Then we'll do a DC sweep. This will give us waveform capabilities.

Next, transient analysis. For transient analysis to be interesting, energy-storing
components are needed, so we'll implement an auld capacitor. Which means numerical
integration - Newton.


I'm not sure where I'll take things from here. There are a lot of interesting
possibilities:
* We could see how Foreign Function Interfaces (FFIs) can be used to import C models
  of MOSFETs without having to re-write the code.
* We could could play with WASM to see if we can run the simulator in a web browser



Target Audience
-----------------------------------------------

People who use SPICE simulators. People who are reasonably comfortable programming.



Choice of Implementation Language
----------------------------------------------

SPICE simulators need to be fast. This is often a combination of the algorithms used
and the implementation language.

C or C++

NGSPICE is in C, by the looks of things.



Prior Art
----------------------------------------------
Internet searches bring up loads of SPICE simulators whose code is open to the
public. 

The author can't personally vouch for any of these, but they are interesting
none the less.

* NGSPICE
* That Java thing



Theory Stuff on the Web
--------------------------------------------
There is some teaching materials on the web. These are links that the author found,
and found useful

* PDF1
* PDF2
* PDF3


Table of Contents
---------------------------------------------

Updated

1. Roadmap [2018-??-??]
2. Getting Started




