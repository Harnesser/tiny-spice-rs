I can pick up primitive device values again, but this time I'm looking
them up in a global parameter table.

Next: lookup the value of a bracket expression with a single identifier.
The value has to be taken from the surrounding scope.


I've a recursive subckt handler working I think. Need tidy up and
support all devices.

But I'm implementing parameters instead...

TODO: Numeric literals for primitive values

TODO: Decide how my circuit elements are to have parameters and
how to store parameters, and how they should be evaluated during
circuit expansion.

