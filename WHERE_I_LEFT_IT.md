I think I want to get this supporting drum machine circuits:

    cargo run ngspice/diff_opamp.spi ; python3 bin/r8n -expr "2,3-4,7-8" waves/diff_opamp/

Maybe have the `plot` command at least write the `r8n` command?

For WAV writing, i need:
1. samples on a grid
2. a module to write WAV files

If i do (1) first, that's ok, it'd need to be done anyway. A module to write
WAV files, or call out to python, i can decide to do that later. So (1) first.
