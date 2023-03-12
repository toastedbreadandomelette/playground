import numpy as np
from time import time
from scipy import linalg, fft
# import pandas as pd
# import polars as pl

sz = 24
# c = np.full((sz, sz, sz), 1.02)
# print(c)
# p = np.array([[[[np.arange(i + j, i + j + sz * (i + j), (i + j))
#              for i in range(1, sz + 1)] for j in range(1, sz + 1)] for k in range(1, sz + 1)] for l in range(1, sz + 1)])
# p = np.array([[np.arange(i, i + sz * (i), (i))
#              for i in range(1, sz + 1)] for j in range(sz)])
p = np.arange(sz)
t = time()

# for x in range(10000):
# s = np.fft.fft2(p)
# sz = (4632647)  # - 131072 - 1
# print(sz, bin(sz))
# for x in range(500):
# print(p)
s = fft.fft(p)
b = fft.dct(p, type=2)
print(s)
print(b)

v = time() - t
# print(p)
print(f'{v}s')
# print(s)
# print(df)
# df.LazyFrame.describe_optimized_plan()
# print(c.shape, c.size, c)
