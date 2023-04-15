import numpy as np
from time import time
sz = 1024
a = np.array([np.arange(sz) for i in range(sz)], dtype=np.float64)
b = np.array([np.arange(i, i + sz) for i in range(sz)], dtype=np.float64)

t = time()
c = np.matmul(a, b)
print(f'{time() - t}s')
