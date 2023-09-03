# Outputs from `tiny-spice-rs`

## Overview
I haven't decided on what I want to do for the outputs yet.

## Waveform Database
My transient analysis, for now, dumps out all of the data into `waves/<circuit-name>/trans.dat`. 

## Command - `print`
Maybe have it just print the last values for transient simulations?

In `ngspice`, the `print` command will plot all the timepoints and values in the
console for transient analysis. This is too noisy for this project.


## Command -  `plot`
Ideally, it would be nice if the `plot` command brought up a window in interactive
mode and plotted to a PNG in batch mode. Far too much to implement for this toy,
though. Do I really want to bring in `matplotlib` and python as a non-dev
dependency to plot stuff?


In `ngspice`, the `plot` command will bring up that terrible waveform viewer.

