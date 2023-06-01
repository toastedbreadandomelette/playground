
class matrix:
    """
    Matrix class.
    >>> a = matrix([[1, 1], [1, 1]]); b = matrix([[1, 1], [1, 1]]); c = a + b; c
    [2, 2]
    [2, 2]
    """
    def __init__(self, mat):
        """
        Constructor
        >>> a = matrix([[1, 2], [1]])
        Traceback (most recent call last):
         ...
        Exception: Each row should have a fixed width
        """
        if not all([len(mat[i]) == len(mat[i+1]) for i in range(len(mat) - 1)]):
            raise Exception('Each row should have a fixed width')
        self.mat = mat
    
    def __repr__(self):
        """
        Console value of matrix
        """
        return '\n'.join([str(row) for row in self.mat])

    def __str__(self):
        """
        Display contents of matrix
        """
        return '\n'.join([str(row) for row in self.mat])

    def __eq__(self, __other):
        """
        Check whether a matrix is equal to another
        """
        return isinstance(__other, matrix) and self.mat == __other.mat

    def __neg__(self):
        """
        Negate the values in matrix
        >>> a = matrix([[1, 1], [2, 2]])
        >>> a = -a
        >>> a
        [-1, -1]
        [-2, -2]
        """
        return matrix([[-val for val in row] for row in self.mat])

    def __add__(self, __other):
        """
        Addition of two matrices
        >>> a = matrix([[1, 1], [2, 2]])
        >>> b = matrix([[1, 1], [2, 2]])
        >>> a + b
        [2, 2]
        [4, 4]
        """
        if isinstance(__other, int) or isinstance(__other, float):
            return matrix([ [self.mat[i][j] + __other for j in range(len(self.mat[i]))] for i in range(len(self.mat))])
        assert(len(self.mat) == len(__other.mat) and len(self.mat[0]) == len(__other.mat[0]))
        return matrix([ [self.mat[i][j] + __other.mat[i][j] for j in range(len(self.mat[i]))] for i in range(len(self.mat))])

    def __iadd__(self, b):
        """
        Addition of two matrices
        >>> a = matrix([[1, 1], [2, 2]])
        >>> b = matrix([[1, 1], [2, 2]])
        >>> a += b
        >>> a
        [2, 2]
        [4, 4]
        """
        if isinstance(b, int) or isinstance(b, float):
            for row in range(len(self.mat)):
                for col in range(len(self.mat[row])):
                    self.mat[row][col] += b
        elif isinstance(b, matrix):
            assert len(self.mat) == len(b.mat) and len(self.mat[0]) == len(b.mat[0])
            for row in range(len(self.mat)):
                for col in range(len(self.mat[row])):
                    self.mat[row][col] += b.mat[row][col]
        return self

    def __sub__(self, b):
        """
        Subtraction of two matrices
        >>> a = matrix([[1, 1], [2, 2]])
        >>> b = matrix([[1, 1], [2, 2]])
        >>> a - b
        [0, 0]
        [0, 0]
        """
        if isinstance(b, int) or isinstance(b, float):
            return matrix([[self.mat[i][j] - b for j in range(len(self.mat[i]))] for i in range(len(self.mat))])
        assert(len(self.mat) == len(b.mat) and len(self.mat[0]) == len(b.mat[0]))
        return matrix([[self.mat[i][j] - b.mat[i][j] for j in range(len(b.mat[i]))] for i in range(len(b.mat))])

    def __isub__(self, b):
        """
        Subtraction of two matrices
        >>> a = matrix([[1, 1], [2, 2]])
        >>> b = matrix([[1, 1], [2, 2]])
        >>> a -= b
        >>> a
        [0, 0]
        [0, 0]
        >>> a += b
        >>> a += b
        >>> a
        [2, 2]
        [4, 4]
        """
        if isinstance(b, int) or isinstance(b, float):
            for row in range(len(self.mat)):
                for col in range(len(self.mat[row])):
                    self.mat[row][col] -= b
        elif isinstance(b, matrix):
            assert(len(self.mat) == len(b.mat) and len(self.mat[0]) == len(b.mat[0]))
            for row in range(len(self.mat)):
                for col in range(len(self.mat[row])):
                    self.mat[row][col] -= b.mat[row][col]
        return self

    def __setitem__(self, key, value):
        """
        Set the value of an item
        >>> a = matrix([[1, 1], [2, 2]])
        >>> a[1,1] = 9
        >>> a
        [1, 1]
        [2, 9]
        """
        assert (isinstance(key, tuple) and len(key) == 2) or isinstance(key, int)
        if isinstance(key, tuple):
            assert isinstance(key[0], int) and isinstance(key[1], int)
            self.mat[key[0]][key[1]] = value
        else:
            self.mat[key] = value

    def __getitem__(self, key):
        """
        Get items in matrix
        >>> a = matrix([[i*5 + j + 1 for j in range(5)] for i in range(5)])
        >>> a[1,1]
        7
        >>> a[:,1:3]
        [2, 3]
        [7, 8]
        [12, 13]
        [17, 18]
        [22, 23]
        >>> a[1:3]
        [6, 7, 8, 9, 10]
        [11, 12, 13, 14, 15]
        >>> b = matrix([[i*4 + j + 1 for j in range(4)] for i in range(4)])
        >>> b[:2,:2]
        [1, 2]
        [5, 6]
        """
        assert (isinstance(key, tuple) and len(key) == 2) or (isinstance(key, slice)) or isinstance(key, int)
        if isinstance(key, tuple):
            row_sel, col_sel = key
    
            assert (isinstance(row_sel, slice) or isinstance(row_sel, int)) and \
                   (isinstance(col_sel, slice) or isinstance(col_sel, int))

            if isinstance(row_sel, slice):
                row_indices = range(*row_sel.indices(len(self.mat)))
                return matrix([self.mat[row][col_sel] for row in row_indices])

            elif isinstance(row_sel, int):
                return self.mat[row_sel][col_sel]

        elif isinstance(key, slice):
            return matrix(self.mat[key])
        
        return self.mat[key]

    def __shape__(self):
        """
        Get the shape of matrix
        """
        return len(self.mat), len(self.mat[0])

    def h_stack(self, second_matrix):
        """
        Horizontally stack the values of second matrix
        >>> a = matrix([[1, 1], [2, 2]])
        >>> b = matrix([[1, 1], [2, 2]])
        >>> a.h_stack(b)
        >>> a
        [1, 1, 1, 1]
        [2, 2, 2, 2]
        """
        assert len(self.mat) == len(second_matrix.mat)
        for row1, row2 in zip(self.mat, second_matrix.mat):
            row1.extend(row2)

    def v_stack(self, second_matrix):
        """
        Stack the values of self on top of second matrix
        >>> a = matrix([[1, 1], [2, 2]])
        >>> b = matrix([[1, 1], [2, 2]])
        >>> a.v_stack(b)
        >>> a
        [1, 1]
        [2, 2]
        [1, 1]
        [2, 2]
        """
        assert len(self.mat[0]) == len(second_matrix.mat[0])
        self.mat.extend(second_matrix.mat)

    def is_square(self):
        """
        Check if matrix is square
        >>> a = matrix([[1, 1], [2, 2]])
        >>> a.__shape__()
        (2, 2)
        >>> b = matrix([[1, 1], [2, 2]])
        >>> a.v_stack(b)
        >>> a.__shape__()
        (4, 2)
        """
        return len(self.mat) == len(self.mat[0])

    def default_multiplication(self, second_matrix):
        """
        Multiplication of two matrices:
        """
        c = []
        m, n, p = len(self.mat), len(self.mat[0]), len(second_matrix.mat[0])
        for row in range(m):
            c.append([sum ([self.mat[row][k] * second_matrix.mat[k][j] for k in range(n)]) for j in range(p)])
        return matrix(c)

    def strassen_multiplication(self, second_matrix):
        """
        Multiplication of two matrices by strassen's method.
        Outperforms naive multiplications for size n >= 200
        Complexity is `O(n^2.807)`

        The condition:
        - If size is odd, we perform the submatrix multiplication by naive
        method (See wikipedia for explaination), note that you can pad zeroes 
        before and remove them after calculation if matrix size is odd and very large
        - Only square matrices are allowed.

        [Some clever adjustments can be made though](https://en.wikipedia.org/wiki/Strassen_algorithm#Implementation_considerations)

        >>> sz = 128
        >>> a = matrix([[i*sz + j + 1 for j in range(sz)] for i in range(sz)])
        >>> b = matrix([[i*sz + j + 1 for j in range(sz)] for i in range(sz)])
        >>> c = matrix([[1,2],[3,4]])
        >>> c.strassen_multiplication(c[:,:])
        [7, 10]
        [15, 22]
        >>> import time
        >>> t1 = time.time()
        >>> c1 = a.default_multiplication(b)
        >>> def_time = time.time() - t1
        >>> t1 = time.time()
        >>> c2 = a.strassen_multiplication(b)
        >>> strassen_time = time.time() - t1
        >>> c1 == c2
        True
        >>> strassen_time < def_time
        True
        """
        # Smaller matrices can be computed faster with normal 
        # multiplication
        if len(self.mat) <= 32:
            return self.default_multiplication(second_matrix)

        row, col = self.__shape__()
        if row % 2 == 0:
            row2 = row // 2
            a, b, c, d = self[:row2, :row2], self[:row2, row2:], self[row2:, :row2], self[row2:, row2:]
            e, f, g, h = second_matrix[:row2, :row2], second_matrix[:row2, row2:], \
                         second_matrix[row2:, :row2], second_matrix[row2:, row2:]
            
            p1 = a.strassen_multiplication(f - h)
            p2 = (a + b).strassen_multiplication(h)
            p3 = (c + d).strassen_multiplication(e)
            p4 = d.strassen_multiplication(g - e)
            p5 = (a + d).strassen_multiplication(e + h)
            p6 = (b - d).strassen_multiplication(g + h)
            p7 = (a - c).strassen_multiplication(e + f)

            c11 = p5 + p4 - p2 + p6
            c12 = p1 + p2
            c21 = p3 + p4
            c22 = p1 + p5 - p3 - p7

            c21.h_stack(c22)
            c11.h_stack(c12)
            c11.v_stack(c21)
            return c11

        return self.default_multiplication(second_matrix)

    def __identity__(self, size):
        """
        Return identity matrix
        """
        return matrix([[1 if i == j else 0 for j in range(size)] for i in range(size)])

    def __set_identity__(self):
        """
        Set self as identity matrix
        """
        assert self.is_square()
        for i, row in enumerate(self.mat):
            for index in range(len(row)):
                row[index] = 1 if i == index else 0

    def __mul__(self, a):
        """
        Multiplication based on type:
        for integer/float: scalar multiplication
        """
        assert len(self.mat[0]) == len(a.mat)
        if isinstance(a, float) or isinstance(a, int):
            return [[a*self.mat[i][j] for j in range(len(self.mat[i]))] for i in range(len(self.mat))]
        else:
            return self.strassen_multiplication(a)

    def __imul__(self, a):
        assert len(self.mat[0]) == len(a.mat)
        if isinstance(a, float) or isinstance(a, int):
            for row in self.mat:
                for index in range(len(row)):
                    row[index] *= a
            return self
        else:
            self = self.strassen_multiplication(a)
            return self

    def __truediv__(self, num: float):
        """
        Divide matrix values be certain scalar value
        >>> a = matrix([[2, 2], [4, 4]])
        >>> a = a / 2
        >>> a
        [1.0, 1.0]
        [2.0, 2.0]
        """
        assert isinstance(num, int) or isinstance(num, float)
        return matrix([[val / num for val in row] for row in self.mat])

    def __itruediv__(self, num: float):
        """
        Divide matrix values be certain scalar value
        >>> a = matrix([[2, 2], [4, 4]])
        >>> a /= 2
        >>> a
        [1.0, 1.0]
        [2.0, 2.0]
        """
        assert isinstance(num, int) or isinstance(num, float)
        for row in self.mat:
            for index in range(len(row)):
                row[index] /= num
        return self

    def __floordiv__(self, num: float):
        """
        Floor Divide matrix values be certain scalar value
        >>> a = matrix([[2, 2], [4, 4]])
        >>> a = a // 2
        >>> a
        [1, 1]
        [2, 2]
        """
        assert isinstance(num, int)
        return matrix([[val // num for val in row] for row in self.mat])

    def __ifloordiv__(self, num: float):
        """
        Divide matrix values be certain scalar value
        >>> a = matrix([[2, 2], [4, 4]])
        >>> a //= 2
        >>> a
        [1, 1]
        [2, 2]
        """
        assert isinstance(num, int)
        for row in self.mat:
            for index in range(len(row)):
                row[index] //= num
        return self

    def __mod__(self, num: int):
        """
        Return matrix values mod values
        >>> a = matrix([[2, 2], [4, 4]])
        >>> a = a % 3
        >>> a
        [2, 2]
        [1, 1]
        """
        assert isinstance(num, int)
        return matrix([[val % num for val in row] for row in self.mat])

    def __imod__(self, num: int):
        """
        Return remainder of each element in matrix
        >>> a = matrix([[2, 5], [3, 8]])
        >>> a %= 5
        >>> a
        [2, 0]
        [3, 3]
        """
        assert isinstance(num, int)
        for row in self.mat:
            for index in range(len(row)):
                row[index] %= num
        return self

    def __pow__(self, num: int):
        """
        Calculate matrix to the power
        >>> a = matrix([[1, 1], [2, 2]])
        >>> a = a**3
        >>> a
        [9, 9]
        [18, 18]
        """
        n, res = self[:,:], self.__identity__(len(self.mat))
        while num > 0:
            if num & 1:
                res *= n
            n *= n
            num >>= 1
        return res

    def __ipow__(self, num: int):
        """
        Calculate matrix to the power
        >>> a = matrix([[1, 1], [2, 2]])
        >>> a **= 4
        >>> a
        [27, 27]
        [54, 54]
        """
        n = self[:,:]
        self.__set_identity__()
        while num > 0:
            if num & 1:
                self *= n
            n *= n
            num >>= 1
        return self

    def __is_identity__(self):
        """
        Check if a matrix is an identity matrix or not.
        """
        row, col = self.__shape__()
        return row == col and \
              all([True if ((i == j and self.mat[i][j] == 1) or (i != j and self.mat[i][j] == 0)) else False for j in range(col) for i in range(row)])

    def reduced_row_echelon_form(self):
        """
        Convert self matrix to a reduced row echelon form
        """
        n = len(self.mat)
        mt = self.mat
        row, lead = 0, 0
        while row < n and lead < 2*n:
            i = row
            # Check whether the lower matrix is reduced to zero.
            # if it is, then return
            while mt[i][lead] == 0:
                i += 1
                if i == n:
                    i = row
                    lead += 1
                    if lead == 2*n:
                        return
            
            # swap the rows.
            mt[i], mt[row] = mt[row], mt[i]

            if mt[row][lead] != 0:
                f = mt[row][lead]
                for c in range(2*n):
                    mt[row][c] /= f

            for j in range(n):
                if j == row:
                    continue
                f = mt[j][lead]
                for k in range(2*n):
                    mt[j][k] -= f*mt[row][k]
            
            row += 1
            lead += 1

    def round_mat(self, digits=6):
        """
        Round off elements of matrices upto certain decimal digits
        """
        row, col = self.__shape__()
        for x in range(row):
            for y in range(col):
                self[x,y] = round(self[x,y], digits)

    def gauss_jordan_inverse(self):
        """
        Inverse of a matrix using gauss - jordan elimination
        method
        >>> import random
        >>> a = matrix([[5, 7, 9], [4, 3, 8], [7, 5, 6]])
        >>> answer = (a.gauss_jordan_inverse()*a)
        >>> answer.round_mat(10)                      # setting float precision
        >>> answer.__is_identity__()
        True
        >>> sz = 20
        >>> b = matrix([[random.randint(1,sz*8) for j in range(sz)] for i in range(sz)])
        >>> b[sz-1,sz-1] += sz*2
        >>> answer = (b.gauss_jordan_inverse()*b)
        >>> answer.round_mat(10)                      # setting float precision
        >>> answer.__is_identity__()
        True
        """
        assert len(self.mat) == len(self.mat[0])
        
        n = len(self.mat)

        # create augmented matrix (set identity and keep adjacent to original matrix): 
        # attach it to the copy of original matrix, such that shape is n * 2n
        aug_matrix = matrix([[(1 if i == j else 0) for j in range(len(self.mat[i]))] for i in range(len(self.mat))])
        mat = matrix(self[:,:].mat)
        mat.h_stack(aug_matrix)

        for row in range(n - 1, 0, -1):
            if mat[row - 1,0] < mat[row,0]:
                mat[row - 1], mat[row] = mat[row], mat[row - 1]

        mat.reduced_row_echelon_form()
    
        return mat[:, n:]

    def strassen_inverse(self):
        """
        Strassens method of inversion.
        Time complexity is the same as strassen multiplication. `O(n^2.807)`

        Source: https://arxiv.org/pdf/1801.04723.pdf

        >>> import random
        >>> import time
        >>> sz = 128
        >>> # randomness cannot be considered okay since there might be a chance for matrix whose det is zero
        >>> b = matrix([[random.randint(-100-sz,sz+100) for j in range(sz)] for i in range(sz)])
        >>> t1 = time.time()
        >>> answer = (b.inverse())
        >>> s_time = time.time() - t1
        >>> answer = answer * b
        >>> answer.round_mat()                 # setting float precision
        >>> id = answer.__is_identity__()
        >>> t1 = time.time()
        >>> answer = b.gauss_jordan_inverse()
        >>> g_time = time.time() - t1
        >>> answer = answer * b
        >>> answer.round_mat()
        >>> # Assuming gauss_jordan method is reliable, if det is zero, then both evaluation should be same
        >>> id == answer.__is_identity__()
        True
        >>> # also, the time difference in gauss_jordan should take more time for bigger matrices
        >>> g_time > s_time
        True
        """
        row, col = self.__shape__()
        # Threshold is kept as 50, below which the naive
        # inverse evaluation is faster.
        if row <= 50:
            return self.gauss_jordan_inverse()

        if row % 2 == 0:
            row2 = row // 2
            a11, a12, a21, a22 = self[:row2, :row2], self[:row2, row2:], self[row2:, :row2], self[row2:, row2:]

            s1 = a11.strassen_inverse()
            s2 = a21.strassen_multiplication(s1)
            s3 = s1.strassen_multiplication(a12)
            s4 = a21.strassen_multiplication(s3)
            s5 = s4 - a22
            s6 = s5.strassen_inverse()
            
            c12 = s3.strassen_multiplication(s6)
            c21 = s6.strassen_multiplication(s2)
            s7 = s3.strassen_multiplication(c21)
            c11 = s1 - s7
            c22 = -s6

            c21.h_stack(c22)
            c11.h_stack(c12)
            c11.v_stack(c21)
            return c11

        return self.gauss_jordan_inverse()

    def inverse(self):
        """
        Calculates the inverse of a matrix
        """
        if not self.is_square():
            raise Exception('Should be a perfect square for finding inverse.')
        return self.strassen_inverse()

if __name__ == '__main__':
    from doctest import testmod
    testmod()
