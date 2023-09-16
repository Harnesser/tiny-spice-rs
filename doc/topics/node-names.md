# Node Names

## Tag 0.7.0
In version 0.7.0 only integers are allowed for node names. This makes building the
node matrix easy as the index to the matrix for a certain node is the node name.
This also has the effect of pushing the voltage sources to the outskirts of the 
matrix:

    +-----------+-+-+
    |           | | |
    |           | | |
    |   Node    | | |
    |  Voltages |V|I|
    |           | | |
    |           | | |
    +-----------+ +-+
    |    V      |V|0|
    +-------------+-+

## Name Scheme

* `gnd` and `GND` are aliases for index 0?
* Case sensitive: `n1` is not the same node as `N1`
* Must start with an ascii letter `[a-zA-Z]`
* Other letters then can be the usual `[_0-9a-zA-Z]`

## Plan

To support subcircuits, this is going to have to change. Subcircuit nodes will be
in the form `<inst>.<inst>.<nodename>`. Dots

* Map names to index in matrix
* For stamping? I'll pass indices around, not names.
* Will there be a need to map index back to node name?

## Open Questions

* Will parsing of hierarchical nodes be allowed in SPICE cards?
* How will I deal with node aliases? This is more important for `print` and `plot`
  commands, I think. And these are not implemented anyway.
