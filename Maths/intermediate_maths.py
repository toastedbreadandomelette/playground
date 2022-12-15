
pi = 3.1415926535897932384626433832795028841971693993751058209749445923078164

def sieve_of_eratosthenes(n) -> list:
    """
    Note that this generates a list of boolean array where ith 
    position denotes whether a number is prime or not

    Optimization: 
    - The list can be reduced by checking only odd numbers.
    - Using bitsets instead of array of boolean
    """
    # Already marked for i = 2
    sieve = [False if (i % 2 == 0 and i > 2 and i != 0) else True for i in range(n+1)]
    sqrt_n = isqrt(n)
    for number in range(3, sqrt_n + 1):
        # If number is marked as prime
        if sieve[number]:
            # Mark it's multiples as not prime
            for marker in range(number * number, n + 1, number):
                sieve[marker] = False
    return sieve

def prime_with_sieve(number_list: list) -> list:
    """
    Return if an integer in the list is prime or not.
    To refer how sieve works, refer sieve_of_eratosthenes function

    >>> prime_with_sieve([2, 11, 22, 33, 41, 68, 97, 8831, 8849, 8850])
    [True, True, False, False, True, False, True, True, True, False]
    """
    sieve = sieve_of_eratosthenes(max(number_list) + 1)
    return [sieve[number] for number in number_list]

def segmented_sieve(left: int, right: int) -> list:
    """
    Generates prime check from left to right
    - Create sieve from 1 to sqrt(right), with primes in the list
    - For each prime in primes list:
        - Get starting value divisible by prime (say st)
        - Mark all the values from st to right (including right) which are divisible 
    by prime.
    """
    upper_limit = isqrt(right)
    sieve = [0 if i % 2 == 0 and i != 2 else 1 for i in range(upper_limit + 1)]
    primes = [2]
    for marker in range(3, upper_limit + 1, 2):
        # If prime
        if sieve[marker] == 1:
            primes.append(marker)
            for mark in range(marker * marker, upper_limit + 1, marker):
                sieve[mark] = 0
    
    seg_sieve = [1] * (right - left + 1)
    for prime in primes:
        """
        Start should be either (prime * prime) if the value lies in the range (left, right),
        else find first value divisible by (prime)
        """
        start, end, skip = max(prime * prime, ((left + prime - 1) // prime) * (prime)), right + 1, prime
        for mark in range(start, end, skip):
            seg_sieve[mark - left] = 0
    if left == 1:
        seg_sieve[1] = 0
    return seg_sieve

def prime_with_segmented_sieve(number_list: list) -> list:
    """
    Calculates the prime within min(number_list) and max(number_list), and evaluates
    primality test for all integers in the list
    >>> prime_with_segmented_sieve([13466917, 20996011, 24036583, 25964952, 30402447, 32582657])
    [1, 1, 1, 0, 0, 1]
    """
    left, right = min(number_list), max(number_list)
    seg_sieve = segmented_sieve(left, right)
    return [seg_sieve[number - left] for number in number_list] 

def linear_recurrence(coefficient: list, base_cases: int, n: int, mod=0) -> int:
    """
    Evaluates the linear recurrence relation of the form:
    ```
    f(n) = a1. f(n - 1) + a2. f(n - 2) + ... + ak. f(n - k), n >= k
         = bi, i < k
    ```
    using matrix exponentiation in O(k^3 . log(n)):
    >>> [linear_recurrence([1, 1, 1], [0, 0, 1], i) for i in range(20)] # tribonacci, first 20 series
    [0, 0, 1, 1, 2, 4, 7, 13, 24, 44, 81, 149, 274, 504, 927, 1705, 3136, 5768, 10609, 19513]
    >>> [linear_recurrence([1, 2], [0, 1], i) for i in range(20)] # pell numbers, first 20 series
    [0, 1, 2, 5, 12, 29, 70, 169, 408, 985, 2378, 5741, 13860, 33461, 80782, 195025, 470832, 1136689, 2744210, 6625109]
    >>> [linear_recurrence([1, 1], [1, 2], i) for i in range(10)] # fibonacci with different initial values
    [1, 2, 3, 5, 8, 13, 21, 34, 55, 89]
    """
    def mat_mul(a: list, b: list, mod: int) -> list:
        """
        Matrix multiplication of two matrices
        """
        c = []
        rowl_a, coll_a, rowl_b, coll_b = len(a), len(a[0]), len(b), len(b[0])
        assert(coll_a == rowl_b)
        if mod == 0:
            for i in range(rowl_a):
                c += [[sum([a[i][k] * b[k][j] for k in range(rowl_b)]) for j in range(coll_b)]]
    
        else:
            c = [[0 for j in range(coll_b)] for i in range(rowl_a)]
            for i in range(rowl_a):
                for j in range(coll_b):
                    for k in range(rowl_b):
                        c[i][j] += a[i][k] * b[k][j]
                        c[i][j] %= mod
    
        return c

    def mat_expo(a: list, exponent: int, mod: int) -> int:
        """
        Matrix exponentiation, similar to mod_power function
        """
        # Identity matrix
        result = [[0 if i != j else 1 for i in range(len(a))] for j in range(len(a))]
        while exponent > 0:
            if exponent & 1:
                result = mat_mul(result, a, mod)
            a = mat_mul(a, a, mod)
            exponent >>= 1
        return result

    def construct_matrix(coefficient: int, base_cases: int) -> list:
        """
        Construct the matrix that evaluates the recurrence relation
        """
        k = len(base_cases)
        matrix = [[0 for i in range(k)] for j in range(k)]
        for x in range(1, k):
            matrix[x][x - 1] = 1
        for x in range(k):
            matrix[x][-1] = coefficient[x]
        return matrix
    
    mat = construct_matrix(coefficient, base_cases)
    mat_n = mat_expo(mat, n, mod)
    answer = mat_mul([base_cases], mat_n, mod)
    return answer[0][0]

def sqrt_bin_search(n: int) -> int:
    """
    Evaluates integer square root by using binary search,
    therefore, takes `O(n)` time.
    >>> sqrt_bin_search(25)
    5
    >>> sqrt_bin_search(100)
    10
    >>> sqrt_bin_search(65)
    8
    """
    if n == 1: return 1
    low, high = 0, n
    while low < high:
        mid = (low + high) // 2
        if mid * mid < n:
            low = mid + 1
        elif mid * mid > n:
            high = mid - 1
        else:
            return mid
    return low

def isqrt(n: int) -> int:
    """
    Integer square root of a number using Heron's method, a 
    special case of Newton's method
    To find the closest square root: 
    ```
    The function f(n) = x^2 - n.
              =>    0 = x^2 - n. 
    ```
    Newton's formula:
    ```
    x(i+1) = x(i) - (f(x(i)) / (d(f(x(i)))/dx(i)))
         = x(n) - ((x(n)^2 - n) / 2 * x(i))
         = 1/2 (x(i) + n / x(i))

    Here (d(f(x(i)))/dx(i)) is derivative of f(x(i)) with respect to x(i)
    With each iteration, the value is computed more accurately.
    ```
    Complexity: O(logn)
    >>> isqrt(2)
    1
    >>> isqrt(4)
    2
    >>> isqrt(5)
    2
    >>> isqrt(30)
    5
    >>> isqrt(25)
    5
    >>> isqrt(20000)
    141
    """
    xn = n // 2
    if xn != 0:
        xn_1 = (xn + n // xn) // 2
        while xn_1 < xn:
            xn = xn_1
            xn_1 = ((xn + n // xn) // 2)
        return xn
    else:
        return n

def sqrt(n: float) -> int:
    """
    Integer square root of a number using Heron's method, a 
    special case of Newton's method
    To find the closest square root: 
    ```
    The function f(n) = x^2 - n.
              =>    0 = x^2 - n. 
    ```
    Newton's formula:
    ```
    x(i+1) = x(i) - (f(x(i)) / (d(f(x(i)))/dx(i)))
         = x(n) - ((x(n)^2 - n) / 2 * x(i))
         = 1/2 (x(i) + n / x(i))

    Here (d(f(x(i)))/dx(i)) is derivative of f(x(i)) with respect to x(i)
    With each iteration, the value is computed more accurately.
    ```
    Complexity: O(logn)
    >>> sqrt(2)         # 1.4142135623730950488016887242096980785696718753
    1.414213562373095
    >>> sqrt(4)         # 2.0
    2.0
    >>> sqrt(5)         # 2.23606797749979
    2.23606797749979
    >>> sqrt(30)
    5.477225575051661
    >>> sqrt(25)
    5.0
    >>> sqrt(20000)
    141.4213562373095
    >>> sqrt(4294967296)
    65536.0
    >>> sqrt(1<<64)
    4294967296.0
    """
    xn = n / 2
    precision = 1e-18

    xn_1 = (xn + n / xn) / 2
    while abs(xn - xn_1) > precision:
        xn = xn_1
        xn_1 = (xn + n / xn) / 2
    return xn

def normalize_angle(x: float) -> float:
    """
    Keep the angles in range from 0 to 2π
    """
    tempx = (2*pi) * (x // (2*pi))
    if x >= 0:
        x -= tempx
    else:
        x += abs(tempx)
        x = 2*pi - x
    return x

def sine(x: float) -> float:
    """
    Compute sine of the angle in radians
    
    First: normalize the angle:
    i.e., x >= 0 and x <= pi

    >>> sine(pi / 2)
    1.0
    """
    x = normalize_angle(x)
    answer, add_by, iter = x, x, 3
    precision = 1e-18
    while True:
        prev = add_by
        add_by *= -(x ** 2)
        add_by /= (iter - 1)
        add_by /= (iter)
        if precision > abs(add_by - prev):
            break
        answer += add_by
        iter += 2
    return round(answer, 16)

def cosine(x: float) -> float:
    """
    Compute cosine of angle in radians
    >>> cosine(pi / 2)
    0.0
    """
    x = normalize_angle(x)
    answer, add_by, iter = 1, 1, 2
    precision = 1e-18
    while True:
        prev = add_by
        add_by *= -(x ** 2)
        add_by /= (iter - 1)
        add_by /= (iter)
        if precision > abs(add_by - prev):
            break
        answer += add_by
        iter += 2
    return round(answer, 16)

def tan(x: float) -> float:
    """
    """
    return sine(x) / cosine(x)

def cotan(x: float) -> float:
    """
    """
    return cosine(x) / sine(x)

def secant(x: float) -> float:
    """
    """
    return 1 / cosine(x)

def cosecant(x : float) -> float:
    """
    """
    return 1 / sine(x)

def sineh(x: float) -> float:
    """
    Compute hyperbolic sine of a number, valid for all values of x
    """
    answer, add_by = x, x
    for iter in range(1, 8):
        add_by *= (x ** 2)
        add_by /= (iter * 2)
        add_by /= ((iter * 2) + 1)
        answer += add_by
    return answer

def cosineh(x: float) -> float:
    """
    Compute hyperbolic cosine of the value
    """
    answer, add_by = 1, 1
    for iter in range(1, 8):
        add_by *= (x ** 2)
        add_by /= (iter * 2 - 1)
        add_by /= ((iter * 2))
        answer += add_by
    return answer

def arcsine(x: float) -> float:
    """
    Calculate inverse of sine function
    >>> round(arcsine(0.5), 6) == round(pi/6, 6)
    True
    """
    if x == 1: return pi/2
    assert abs(x) < 1
    add_by, answer = x, x
    for iter in range(1, 10):
        add_by *= (x**2)
        add_by *= (2*iter - 1)
        add_by /= (2*iter)
        answer += (add_by/(2*iter+1))
    return answer

def arccosine(x: float) -> float:
    """
    Compute inverse cosine
    """
    return pi / 2 - arcsine(x)

def arctan(x: float) -> float:
    """
    Compute inverse tangent
    >>> round(arctan(1), 6)
    0.785398
    >>> pi/4
    0.7853981633974483
    >>> round(arctan(1), 6) == round(pi / 4, 6)
    True
    """
    def alternate(x: float) -> float:
        """
        Using Leonhard Euler series since convergence
        is faster.
        [See here](https://en.wikipedia.org/wiki/Inverse_trigonometric_functions#Infinite_series)
        """
        answer = x / (1 + x*x)
        add_by, pow_2 = answer, 4
        for iter in range(1, 25):
            add_by *= (x*x)
            add_by *= (iter*iter)
            add_by *= pow_2
            add_by /= (1+x*x)
            add_by /= ((iter*2+1)*(iter*2))
            answer += add_by
        return answer

    flag = False
    if abs(x) >= 1:
        flag = True
        x = 1/x
    if flag:
        return (pi/2 if x > 0 else -pi/2) - alternate(x)
    return alternate(x)

def e_power(power: float) -> float:
    """
    Compute e to the power x, using taylor series:
    
    >>> e_power(2)
    7.389056098925863
    >>> e_power(10.24)
    28001.125926231496
    >>> e_power(24.567)
    46699532926.622795
    """
    answer, add_by, iter = 1, 1, 1
    precision = 1e-10
    while True:
        prev = add_by
        add_by *= power
        add_by /= iter
        if iter > 10 and precision > abs(prev - add_by):
            break
        iter += 1
        answer += add_by
    return answer

def lnp(x: float) -> float:
    """
    Actual algorithm to calculate the logarithm of a
    number x, where abs(x) < 1
    """
    term = (x - 1) / (x + 1)
    precision = 1e-18
    answer, add_by, iter = term, term, 3
    while True:
        prev = add_by
        add_by *= term ** 2
        if abs((add_by / iter) - (prev / (iter - 2))) < precision:
            break;
        answer += (add_by / iter)
        iter += 2
    return 2 * answer

def ln(x: float) -> float:
    """
    >>> round(ln(2.718281828), 6)
    1.0
    >>> round(ln(7.3890560703259105), 4)
    2.0
    """
    a, pow_2, idx = x, 1, 0
    while a > 1:
        a /= 2
        idx += 1
    return lnp(a) + idx * lnp(2)

if __name__ == "__main__":
    from doctest import testmod
    testmod()
