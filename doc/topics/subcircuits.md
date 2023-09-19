# Subcircuits


## Recursion
Expand downwards from the top. Node names and instance names of expanded circuit
elements get the usual joining of instance names with `.`.
At each subcircuit level, add node aliases for the portnames that take their
`NodeId` from the level above.

## Parameters

Examples:

```spice
    .subckt myfilter in out rval=100k cval=100nF
      cload in out {cval}
      rload in out {rval}
    .ends myfilter
    
    Xfilt a b myfilter rval=10k cval=10nF
```

Don't allow spaces either side of the `=` to make it easier to parse, I think.
Circuit datastructures now have to know what parameters they have, what the 
default should be and what the actual assigned value of the parameter is.

To pass parameters down the hierarchy, do we need to parse simple bracket 
expressions. Will have to do this anyway.

```spice
    Xfilt a b myfilter rval={rval_from_above}
```

And does that mean the toplevel has to support `.param` statements?


