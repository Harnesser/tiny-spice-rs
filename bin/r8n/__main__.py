#! /usr/bin/env python3
""" Plot results from a regression

Usage: 
    r8n <path>                 - plot all waveforms in directory
    r8n -expr "1" <path>       - take col 1 from each data file
    r8n -expr "3-2" <path>     - plot (col 3 - col 2) from each data file
    r8n -expr "1-2,3-4" <path> - plot (col 1 - col 2) and (col3 - col4) from each file


Expected format of waveform files:

    1st row     : Names
    2nd row     : Units
    rest        : Data

    Column "0" (1st) = X axis
    Use -expr to select the Y axis column. Default is "2" (3rd column)
"""

__why__ = """ 
Why am I not using KST2? 
* Programmability
* Can't set data format from the command line?
"""

import os
import sys

import matplotlib.pyplot as plt

import waveforms

# get list of data file names
if len(sys.argv) < 2:
    print("Need to supply a directory name")
    sys.exit(10)

# parse arguments
expect_expr = False
exprs = ["2"]
for arg in sys.argv[1:]:

    if expect_expr:
        exprs = arg.split(',')
        expect_expr = False
    else:
        if arg == '-expr':
            expect_expr = True
        else:
            dirname = arg
        

if expect_expr:
    print("Expected a plot expression")
    sys.exit(15)

# Check that directory name given actually exists
if not os.path.isdir(dirname):
    print("Not a directory: {}".format(dirname))
    sys.exit(20)

filenames = []
for filename in os.listdir(dirname):
    if not filename.endswith('.dat'):
        continue
    filenames.append(filename)

if len(filenames) == 0:
    print("Did not find any *.dat files")
    sys.exit(30)

filenames.sort()

# initialise plot
# (nothing)
colours = [
        'black',
        'red',
        'blue',
        'darkgreen',
        'orange',
        'purple',
        'pink',
        ]

colours = [
        '#0033FF',
        '#339933',
        '#Ff9900',
        '#CC0000',
        ]

# plot all the data
for i in range( len(filenames) ):
    for (j,expr) in enumerate(exprs):
#for i in range( 10 ):
        filename = filenames[i]
        wv = waveforms.load( os.path.join(dirname, filename), expr=expr )
        colour = colours[j%len(colours)]
        plt.plot(wv.x, wv.y, color=colour)
        plt.plot(wv.x[-1], wv.y[-1], 'o', color=colour)

# show
plt.show()


