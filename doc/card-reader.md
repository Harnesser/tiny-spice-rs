* SPICE Card Reader

How SPICE compatible do I want this to be?

1. First line is always a comment
2. Comment lines start with "*"
3. Skip blank lines

4. <value> is floating-point, or decimal or engineering notation
5. <node> is a string
6. <ident> is a string

7. "0", "gnd", and "GND" are synonyms for the ground reference node
8. R<ident> <node> <node> <value>
9. I<ident> <node> <node> <value>
10. V<ident> <node> <node> <value>
11. D<ident> <node> <node>

12. .option <VAR>=<value>
13. .tran <time> <time> <time>
14. .op

Plot? v(<node>) or v(<node>, <node>) i(<node>, <pin>) #branch?
 

