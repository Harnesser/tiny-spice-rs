#! /usr/bin/env python3

import math

def calc_sine(amp, freq, t):
    return amp * math.sin(2.0 * math.pi * freq * t)


print(calc_sine(2.0, 1e3, 1e-6/4.0) * 10)

