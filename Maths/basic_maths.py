from intermediate_maths import isqrt

def first_n_sum(n: int) -> int:
    """
    Returns the sum of first n positive integers
    >>> first_n_sum(10)
    55
    >>> first_n_sum(100)
    5050
    >>> first_n_sum(9)
    45
    """
    return (n * (n + 1)) // 2

def first_n_squared_sum(n: int) -> int:
    """
    Returns the sum of first n squared positive integers
    >>> first_n_squared_sum(10)
    385
    >>> first_n_squared_sum(100)
    338350
    >>> first_n_squared_sum(9)
    285
    """
    return (n * (n + 1) * (2*n + 1)) // 6

def power(number, exponent: int) -> int:
    """
    Evaluates number raise to exponent in O(logn), where logn is logarithm of number
    n to the base 2

    >>> power(2, 20)
    1048576

    >>> power(33, 39)
    166741481249649316381381371919564410409316111826611678288097

    >>> power('abc', 38)
    Traceback (most recent call last):
        ...
    TypeError: Number is of type <class 'str'>
    """
    if type(exponent) is not int:
        raise TypeError('Number is of type %s' % type(number))
    if type(number) is not int and type(number) is not float:
        raise TypeError('Number is of type %s' % type(number))
    result = 1
    while exponent > 0:
        if exponent & 1:
            result *= number
        number *= number
        exponent >>= 1
    return result

def mod_power(number, exponent: int, mod: int) -> int:
    """
    Evaluates number raise to exponent in O(logn), where logn is logarithm of number
    n to the base 2

    >>> mod_power(2, 20, 100)
    76

    >>> mod_power(33, 39, 1000000007)
    322956718

    >>> mod_power('abc', 38, 23)
    Traceback (most recent call last):
        ...
    TypeError: Number is of type <class 'str'>
    """
    if type(exponent) is not int:
        raise TypeError('Number is of type %s' % type(number))
    if type(number) is not int and type(number) is not float:
        raise TypeError('Number is of type %s' % type(number))
    result = 1
    while exponent > 0:
        if exponent & 1:
            result *= number
            result %= mod
        number *= number
        number %= mod
        exponent >>= 1
    return result

def mod_inverse(number: int, mod: int) -> int:
    """
    Compute mod inverse of a number
    """
    return mod_power(number, mod - 2, mod)

def gcd(a: int, b: int) -> int:
    """
    Calculates GCD of two integers a and b
    >>> gcd(10, 20)
    10
    >>> gcd(80, 96)
    16
    >>> gcd(31242132, 398325)
    3
    """
    return a if b == 0 else gcd(b, a % b)

def lcm(a: int, b: int) -> int:
    """
    Calculates LCM of two integers a and b
    >>> lcm(10, 20)
    20
    >>> lcm(80, 96)
    480
    """
    return (a // gcd(a, b)) * b

def extended_euclidean_gcd(a: int, b: int):
    """
    The algorithm represents the gcd(a, b) in the form
    a.x + b.y = gcd(a, b): A form of diophantine equation
    ```
    Starting 
    x, y   = 1, 0
    x1, y1 = 0, 1
    a1, b1 = a, b
    ```
    if b is zero a is the answer, else
    new points for a1, b1
    ```
    => x, x1 = x1, x - (a1 // b1) * x1
    => y, y1 = y1, y - (a1 // b1) * y1
    => a1, b1 = b1, a1 % b1
    ```
    """
    pass

def prime(n: int) -> bool:
    """
    Boolean check whether a number is prime
    >>> prime(2)
    True
    >>> prime(1001)
    False
    >>> prime(104351)
    False
    >>> prime(104729)
    True
    """
    sqrt_n = isqrt(n)
    # Divisible by 2 check
    if not(n & 1) and n != 2:
        return False
    for testn in range(3, sqrt_n + 1, 2):
        if n % testn == 0:
            return False
    return True

def pascal_triangle(n: int) -> list:
    """
    Returns first n + 1 rows of pascal triangle
    >>> pascal_triangle(1)
    [[1], [1, 1]]
    >>> pascal_triangle(3)
    [[1], [1, 1], [1, 2, 1], [1, 3, 3, 1]]
    >>> pascal_triangle(5)
    [[1], [1, 1], [1, 2, 1], [1, 3, 3, 1], [1, 4, 6, 4, 1], [1, 5, 10, 10, 5, 1]]
    """
    pascal_tr = [[1]]
    for row in range(1, n + 1):
        pascal_tr += [[1] + [(pascal_tr[-1][i] + pascal_tr[-1][i - 1]) for i in range(1, row)] + [1]]
    return pascal_tr

def number_factors(n: int) -> list:
    """
    Calclates all factors in `O(sqrt(n))` excluding 1 and n
    >>> number_factors(24)
    [2, 3, 4, 6, 8, 12]
    >>> number_factors(25)
    [5]
    >>> number_factors(36)
    [2, 3, 4, 6, 9, 12, 18]
    """
    sqrt_n = isqrt(n)
    factors = [factor for factor in range(2, sqrt_n + 1) if n % factor == 0]
    rem_factors = [n // factor for factor in factors if factor * factor != n]
    rem_factors.reverse()
    return factors + rem_factors

def factorial(n: int) -> int:
    """
    Returns the factorial of a number
    Factorial is defined as 
    ```
    f(n) = n! = n x (n-1) x (n-2) x ... x 3 x 2 x 1
    f(0) = 0! = 1  (special condition)
    ```
    >>> factorial(7)
    5040
    >>> factorial(8)
    40320
    """
    return 1 if n <= 1 else n * factorial(n - 1)

def fibonacci_recursive(n: int) -> int:
    """
    Solves fibonacci recurrence relation in O(2^n)
    Note this is not ideal to solve for higher n (for n > 30),
    Since each of f(n - k) term is expanded till n == 1.
    
    ```
    f(n) = f(n - 1) + f(n - 2);    n > 1
         = 1;                      n == 1
         = 0;                      n == 0
    ```
    >>> fibonacci_recursive(6)
    8
    >>> fibonacci_recursive(10)
    55
    """
    return n if n <= 1 else (fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2))

def fibonacci_rec_dynamic(memo: dict, n: int) -> int:
    """
    Calculates fibonacci, but has memo to remember calculations.
    Please refer `fibonacci_recursive`. The motive of memo is to 
    calculate once and store it.

    f(n) = f(n - 1) + f(n - 2) iff n > 1

    f(n - 1) is evaluating f(n - 2) and is stored. When f(n - 2) 
    is evaluated it can be retrieved directly from memo, reducing
    the tree calls.

    This is much faster as memo[n] can retrieve in O(1), making overall
    complexity O(n)
    >>> fibonacci_rec_dynamic({}, 20)
    6765
    >>> fibonacci_rec_dynamic({}, 40)
    102334155
    >>> fibonacci_rec_dynamic({}, 100)
    354224848179261915075
    """
    memo[n] = (memo[n] if n in memo else n if n <= 1 else (fibonacci_rec_dynamic(memo, n - 1) + fibonacci_rec_dynamic(memo, n - 2)))
    return memo[n]

def fibonacci_mat_expo(n: int, mod=0) -> int:
    """
    Returns fibonacci in O(logn) time
    
    >>> fibonacci_mat_expo(100, 0)
    354224848179261915075
    >>> fibonacci_mat_expo(100, 1000000007)
    687995182
    >>> fibonacci_mat_expo(199, 0)
    173402521172797813159685037284371942044301
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
        Matrix exponentiation, similar to power function, but with mod
        """
        # Identity matrix
        result = [[0 if i != j else 1 for i in range(len(a))] for j in range(len(a))]
        while exponent > 0:
            if exponent & 1:
                result = mat_mul(result, a, mod)
            a = mat_mul(a, a, mod)
            exponent >>= 1
        return result

    fib_mat = [[0, 1], [1, 1]]
    return mat_expo(fib_mat, n, mod)[0][-1]

def phi(n: int) -> int:
    """
    Phi: Also known as euler totient function calculates
    number of integers < n which are co-prime to n. 
    
    => `gcd(i, n) = 1` for all `1 <= i < n`

    Some observations:
    ```
    phi(p1*p2) = phi(p1) * phi(p2), where `p1` and `p2` are relatively prime
               = phi(p1) * phi(p2) * d/phi(d), when `p1` and `p2` are not co-prime,
    
    where d = gcd(p1, p2)
    ```
    
    >>> phi(7)
    6
    >>> phi(41)
    40
    >>> phi(41*7) # relatively prime
    240
    >>> phi(16)
    8
    >>> phi(9)
    6
    >>> phi(16*9) # relatively prime
    48
    >>> phi(21)
    12
    >>> phi(18)
    6
    >>> phi(21*18) # should be 12*6*(gcd(21, 18) = 3)/(phi(3) = 2) = 108
    108
    """
    result, i = n, 2
    if n & 1 == 0:
        while n & 1 == 0:
            n >>= 1
        result -= (result >> 1)
    i += 1
    while i * i <= n:
        if n % i == 0:
            while n % i == 0:
                n //= i
            result -= result // i
        i += 2
    if n > 1:
        result -= result // n
    return result

if __name__ == "__main__":
    from doctest import testmod
    testmod()
