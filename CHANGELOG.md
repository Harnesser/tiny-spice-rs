# Changelog

## [Unreleased] Non-Linear Solver
DC operating point of simple circuits with: non-linear diode model; current 
sources and resistors.

### Added
- Diode model (forward biased)
- Newton-Raphson solver (with horrible hack for convergence)


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
