## Bugs ##
[ ] V with PWL doesn't barf
[ ] test_ird comes up with a nonsense if Isat = 1e-12
[ ] parameters won't work in I and V sources

## Fixed ##
[X] `gnd` in subcircuits not treated as a global
[X] bad subcircuit instantiations can cause stackoverflows
    - panic if we don't find a subcircuit definition
[X] Running the robustness loop can cause an infinite loop
[X] Diodes don't converge if input reduces to around 0V
[X] Some diode-bridge + RC load tests fail
    - 3pF cap across diodes fixes

