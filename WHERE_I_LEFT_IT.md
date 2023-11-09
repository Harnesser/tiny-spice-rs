I think I want to get this supporting drum machine circuits:

    cargo run ngspice/diff_opamp.spi ; python3 bin/r8n -expr "2,3-4,7-8" waves/diff_opamp/

Maybe have the `plot` command at least write the `r8n` command?

Have a .lib command that can read in a spice deck, but ignore the
toplevel instantiations and `.control` blocks. Only read the
subcircuit definitions? This would allow me to play with subckts
locally - it's kinda like allowing blocklevel tests.



