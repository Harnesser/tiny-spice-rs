Groundwork
==========================================================

Now that we've got our basic toolchains running, let's figure out where to 
start with our simulator. And to do that, we need to define the problem
we're solving.

At it's heart, circuit simulation is all about [MNA][MNA-wiki] - or modified
nodal analysis. 

Quick MNA recap:
* basic nodal analysis can be used to calculate the node voltages in 
  circuits with current sources and resistors using [KCL][KCL-wiki].
* Modified Nodal Analysis extends this idea to deal with voltage sources. MNA
  calculates, or "solves" the current through the voltage sources, as well as
  the other node voltages.

If you need to, have a read of the [QUCs documentation][MNA-QUCs] for a good
introduction/refresher on MNA - the following crap will make more sense if
you do.

So our problem is that we need to write something that lets our computers
know how to convert a netlist into a matrix representing linear equations,
and then get it to solve this matrix for us.

Since the linear equation solver is the heart of this application, I'm going
to start with its implementation and grow everything around this seed. I tried
to mix more metaphors in that previous sentance.


Plan for Getting a DC Solution
-------------------------------
Loads of SPICE analyses need to get a DC solution to start with. The easiest,
and to me minimally interesting, circuit solver starts with a DC solution for
circuits made up of the following elements:

As mentioned in the outline, we're going to start off with computing the DC 
solution for simple circuits that are made up of the following circuit elements:

* `I` - current sources
* `R` - resistances
* `V` - voltage sources

The method for getting a DC solution is dead simple.

1. Create the node matrix that represents the circuit we want to solve
2. Run Guassian Elimination on that matrix

Let's explore what kind of datastructures we'll need to implement to get a
DC solution.


The Circuit Matrix
------------------
Here's a temporary circuit we're dealing with - just for a concrete example.

          R    R  
       I     V    R

This circuit has 4 nodes including ground (aka node `0` or ground). We have 5
circuit elements:

* Current source `I1` is connected between node `1` and ground.
* Voltage source `V1` is connected between ground and node `2`.
* Resistor `R1` is connected between nodes `1` and `2`.
* Resistor `R2` is connected between nodes `2` and `3`.
* Resistor `R3` is connected between nodes `3` and ground.

We have 3 non-ground nodes, and 1 voltage source, so our circuit matrix for
MNA is going to be a 4x4 matrix, and our current matric is 4x1:
  
    -                  -   -   -
   | 0.0  0.0  0.0  0.0 | | 0.0 |
   | 0.0  0.0  0.0  0.0 | | 0.0 |       G I V
   | 0.0  0.0  0.0  0.0 | | 0.0 |       V V I
   | 0.0  0.0  0.0  0.0 | | 0.0 |
    -                   -  -   -

We'll used floating-point numbers because our values and solutions will be in 
fractions of Volts and Amps.

The top-left 3x3 sub-matrix is conductance (Siemens, or 1/Resistance). Which
I think is interesting because no connection between two nodes can be
represented as 0.0 S rather than infinity R.

For computations, the U vector is usually considered as an additional column
to the connection matrix.



Circuit Elements
----------------
The circuit elements are "stamped" on to the circuit matrix. 

If you go through the maths, each different circuit element leaves a different
"stamp" on the circuit matrix.

For example, the current through a resistor depends on the value of the 
resistance, and the two voltages at each end of the resistor. Since the resistor
only has two nodes, the current through one of its nodes has to be the same as the 
current through the other. But one goes into the node, and one comes out of the 
node, so the arithmetic signs will change.

Injected, extracted from the other node. I'm using injected and extracted, because
other elements are no doubt connected to the terminals of the resistor.

We've (v1 - v2)/R going in one node, and out of the other.

An important thing to note is that the stamp "falls off the edge of the world"
if ground is involved.


Solving
-------
Once we've set up the circuit matrix and stamped it with all the components in the
circuit, it's ready to solve.

A bit of [Gaussian elimination][GaEl] solves the node voltages and the current 
through the voltage source.


Output
------
As a final step, we'll need to show our customers the simualated values of nodes in
their circuit. We'll just print out all the node voltages, line-by-line.


Program Things Needed
---------------------
For our program, we'll need:

1. A matrix. We'll use a vector of vectors for a 2D array
2. Functions that know how to stamp the matrix for: I, V & R.
3. A Function that implements Gaussian elimination.


Need to know the index of a node. Look up table that maps a string to a node index.

Modern SPICE allows node names that have letters in them! The original version only
allowed numbered nodes, with `0` being ground, and ground begin global. Modern
programming languages with their Unicode support will allow for emoji net names, and
the sooner all the major CAD companies implement this, the better :| .








References
----------

  [MNA-QUCs]: http://qucs.sourceforge.net/tech/node14.html
  [MNA-wiki]: https://en.wikipedia.org/wiki/Modified_nodal_analysis
  [KCL-wiki]: https://en.wikipedia.org/wiki/Kirchhoff%27s_circuit_laws#Kirchhoff's_current_law_(KCL)
  [GaEl-wiki]: https://en.wikipedia.org/wiki/Gaussian_elimination