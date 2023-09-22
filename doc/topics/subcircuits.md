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

Parameters are wired like nodes - its just that there is some elaboration-time
calculations that have to be done? So add `X.rc_load.rval` to the expanded `ckt`
datastructure `.params` dictionary? What if a node name clashes with a parameter
name? Or are they in different "namespaces"? Which would be a first for SPICE...

When realising a subcircuit during the expansion, working from toplevel down,
add parameter overrides. The question is then how these parameter overrides get
into the `Element` value fields. The `Element` parsing would now have to parse
`Expression`s instead of `f64`s. But all the engine code calls into `.value`
directly?

If I ape the `NodeId` node-aliasing scheme, it seems it'll be easy to get local
parameter values in each subcircuit. The problem is going to be how to set the
values of Rs and Cs in that. They need to know the brace expressions, and I'm
going to have to figure out how to evaluate brace expressions to an `f64` when
realising subcircuits. Which means `Elements` are going to have to know what
their parameters are. They are implicit in `R` and `C` - for example, there's
no `Rload n1 n2 value={100k}` - it's just `Rload n1 n2 100k`.

