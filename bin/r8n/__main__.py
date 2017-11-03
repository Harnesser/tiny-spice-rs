#! /usr/bin/env python3
""" Plot results from a regression

Usage: 
    r8n <path>          - plot all waveforms in directory


Expected format of waveform files:

    1st row     : Names
    2nd row     : Units
    rest        : Data

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

dirname = sys.argv[1]
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

# plot all the data
for i in range( len(filenames) ):
    filename = filenames[i]
    wv = waveforms.load( os.path.join(dirname, filename), col=2 )
    plt.plot(wv.x, wv.y)

# show
plt.show()


