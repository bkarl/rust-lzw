import numpy as np

simple_input = np.array([1,2,3], dtype=np.uint8)
simple_input.tofile("simple_123.dat")
simple_input.tofile("simple_123.dat.orig")
