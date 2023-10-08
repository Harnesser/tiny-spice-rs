# Changelog

## [0.9.0] PWL and Inverters <<UNRELEASED>>
I want tiny-spice-rs to help with my drum machine project. This checkun
adds support for PWL voltage sources, and for a model of an inverter
so I can simulate a twin-T oscillator.

### Added
- PWL voltage sources
 * r > 0 not supported

### Fixed
- Panic now if subcircuit definitions are not found. Otherwise this can
  cause a stack overflow.
- Treats `0`, `GND` and `gnd` within subcircuits as the global nodes
  they are.



## [0.8.0] Subcircuits
Subcircuits! See the circuit `ngspice/param_fullwave_rectifier.spi`.

### Added
- Subcircuits with are supported
- Basic Bracket Expression with 1 identifier supported:
 * e.g. `cval={top_cval}`

### Changed
- Nodes can have names now, but integers still work



## [0.7.0] SPICE Deck Reader

### Added
- Toplevel program `tiny-spice` 
- Sinewave Voltage Source
- SPICE deck reader
 * Components: I, V, R, D, C

### Changed
- Configuration object to hold parameters for analyses
- More useful info in `README`

### Fixed
- Sort `test_results.log`
- Give tests unique names



## [0.6.0] Capacitors
Deal with capacitors

### Added
- Capacitor model for DC operating point analysis
- Integration method - backward euler built into cap linearisation

### Changed
- Analysis functions return stats
- Use Colon & Nagel method to limit the diode overvoltaging
- Diode model now uses ::new() method for instantiations
- `f32`s now `f64`s. This helps convergence A LOT

### Fixed
- Convergence problems with diode networks



## [0.5.0] Transient Analysis
Added iteration count based transient analysis.

### Added
- Iteration count based time marching algorithm form SPICE2
- Sinusoidal current source
- Write names, units & waveform data to a file - to be read with KST
- Transient analysis works with the diode bridge circuit!

### Changed
- Extracted the convergence test to a seperate function

### Fixed


## [0.4.0] Non-Linear Solver
DC operating point of simple circuits with: non-linear diode model; current 
sources and resistors.

### Added
- Diode model (forward biased)
- Newton-Raphson solver

### Changed
- Added proper testing for different circuits


## [0.3.0] Gaussian Elimination with Partial Pivot
Use Guassian Elimination with partial pivot as in Wikipedia algorithm. This
should help with numerical stability.

### Changed
- Gaussian Elimination now works from the column with the biggest `abs()`.
  This helps with numerical stability, apparently.


## [0.2.0] Modified Nodal Analysis
Can handle independent voltage sources now by incorporating Modified Nodal
Anaysis (MNA) techniques. Solver didn't have to be changed!

### Added 
- Circuit builder: voltage sources
- Started Glossary

### Changed
- Matrix now constructed in MNA format


## [0.1.0] DC Operating Point
Can find DC operating point of simple circuits consisting of current sources and
resistors.

### Added 
- Circuit builder: resistors and current sources
- Naive Guassian Elimination
- Back-Substitution

