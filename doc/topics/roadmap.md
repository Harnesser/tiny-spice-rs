# Roadmap for `tiny-spice-rs`

## Drum Machine Support #2 (v0.10.0)
* .lib support
    - (backport lib support to diodes?) (did i get confused with `.model`?)
* Wav file output
   - I will need to get waveforms "on the grid" for this, probably.

## Drum Machine Support #3 (v0.11.0)
* npn support
* throw a profiler at things?

## Longer Term
* DC Sweeps
* Dependent Sources?
* Support a list of analyses in `.control`

## Far Future
* Reciprocity, (maybe needs something like noise anaylsis first?)

## Done

### Misc.
* Implement subcircuits.
* Figure out what to do with `print` and `plot`
    - ignoring them for now
* Expand the allowed netnames to inculde ascii characters
    - can still use "integers" if you want

### Drum Machine Support #1 (v0.9.0)
* PWL voltage source
* Operational Amplifier Model
* (sources should support bracket expressions, not just R & C)
