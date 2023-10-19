# Roadmap for `tiny-spice-rs`

(bugfix release? - gnd in subckts) (v0.8.1)

## Drum Machine Support #1 (v0.9.0)
* PWL voltage source
* Operational Amplifier Model
* (sources should support bracket expressions, not just R & C)

## Drum Machine Support #2 (v0.10.0)
* npn support
* .lib support
 - (backport lib support to diodes?)

## Drum Machine Support #3 (v0.11.0)
* Wav file output
 - I will need to get waveforms "on the grid" for this, probably.
* throw a profiler at things?


## Longer Term
* DC Sweeps
* Dependent Sources?
* Support a list of analyses in `.control`


## Far Future
* Reciprocity, (maybe needs something like noise anaylsis first?)


## Done
* Implement subcircuits.
* Figure out what to do with `print` and `plot`
 - ignoring them for now
* Expand the allowed netnames to inculde ascii characters
 - can still use "integers" if you want
