# Circuit Representation

## SPICE Netlist Representation
A SPICE netlist is basically a graph:

### Passives
Resistors, capacitors, current sources, voltage sources, inductors, etc.

    <vertex kind><vertex name> <edge1> <edge2> <vertex value>

    I1 1 0 3
    R1 1 2 10
    R2 2 0 5


### Diodes
I'd need a model.

    <D><vertex name> <edge1> <edge2> <vertex value>

    D1 1 0 3


## Datastructures

Datastructures in the SPICE engine are:

We're going to identify all the nodes in the circuit with unique integers. As
these integers are used to index various datastructures, we'll `usize` them.

We'll keep the next available node identifier in a `usize` `next_id`.

As we read in the string name of each node, we'll copy the string in vector,
indexed by the node identifer.

To facilitate reverse lookup, we'll have a hashmap that takes a string as
an argument and returns an node identifier `usize`.

1. hashmap[string] = usize
2. vector of strings indexed by node id
3. vector[usize] = vector of usize

The end-game for this is to create a matrix representing the circuit that I can
do Gaussian Reduction and back-to-front solving on. The steps are:


1. Circuit graph, vertex and connecting nodes.
  a. keep node count
  b. id each node we haven't seen (u32)
  
  b. Vec of strings indexed by node id
2. Build a matrix of the KCL equations for each node in the design.
3. Gaussian Reduction
4. Solve back-to-front, using Newton-Raphson on non-linear bits
5. Print out node voltages

I don't think I have to write equations for the ground node.

I think I need an enumeration of vertex kinds: Resistor, ISource etc.


