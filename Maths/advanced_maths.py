from intermediate_maths import isqrt

def linear_recurrence_2(coefficient: list, base_cases: int, n: int, mod=0) -> int:
    pass

def diophantine_equation_pell(N: int) -> int:
    """
    Solving pell's equation, the following solution tries to find the
    minimum integer solutions of `x, y`, using Chakravala's Method.
    Optimization: 
    - Values of m can be easily evaluated based on values on
    values of a + m*b, instead of iterating and checking all m.

    ```
    x^2 - y^2 . N = 1, the triples being(x, y, 1)
    e.g., For x^2 - 61.y^2 = 1,
    ```
    - Writing this as a^2 - N . b^2 = k, triples being (a, b, k)

    ```
        x^2 - 61.y^2 = 1 => find trivial solution:
        a^2 - 61.y^2 = k  - (equation 1)
    =>  8^2 - 61.1   = 3 => triples = (8, 1, 3)
    ```
    - Compose values (m, 1, m^2 - N) with the triples (8, 1, 3)

    ```
    Compose the values as ((am + Nb), (a + bm), k(m^2 - N))
    i.e., ((8m + 61), (8 + m), 3(m^2 - 61))
    ```
    - reduce the values by Bhaskar's lemma
    
    ```
    Divide by k2 = 3^2 = 9 in equation 1, we get triple as
    ((8m + 61) / 3, (8 + m) / 3, (m^2 - 61)/3)
    
    Finding nearest m such that (8 + m) is divisible by 3 and abs(m^2 - 61) is minimal.

    => m = 7, ((8m + 61) / 3, (8 + m) / 3, (m^2 - 61)/3)
    (39, 5, -4)
    ```
    Repeating this process till we get the solution.
    >>> diophantine_equation_pell(61)
    (1766319049, 226153980)
    >>> diophantine_equation_pell(67)
    (48842, 5967)
    >>> diophantine_equation_pell(661)
    (16421658242965910275055840472270471049, 638728478116949861246791167518480580)
    >>> diophantine_equation_pell(9949)
    (23551019614858223475933893515741198183163217312913587552899320396564478041197360918469501097146448985821854465768234479384482435117587576296319428592757548743265811454938493105633433315887574461850060798834186249, 236113054062810988826514929828649213339688520849720684015415366388626019230322623673232286474879711003505448178417385617250641629212134427833135509077013929303770208680820795381507114806491325360400076633910900)
    """
    b = 1
    a = isqrt(1 + N)
    if a * a == N + 1:
        return (a, 1)
    else:
        a += 1
        k = a * a - N
        # Next composite value = ((am + Nb), (a + bm), k(m^2 - N))
        i = 0
        next_composite_value_reduced = lambda a, b, k, m: ((a * m + N * b) // abs(k), (a + m*b) // abs(k), (m * m - N) // k)
        while True:
            m = 1
            prev_m = -1
            while True:
                B, K = a + m*b, m * m - N
                if B % abs(k) == 0 :
                    if K > 0:
                        if abs(K) > abs((m-1)**2 - N) and prev_m != -1:
                            m = prev_m
                        a, b, k = next_composite_value_reduced(a, b, k, m)
                        break
                    if K != 0:
                        prev_m = m
                m += 1
            if a * a - N * b * b == 1:
                break
            i += 1
    return a, b

def partitioning_numbers(n: int) -> int:
    """
    This function evaluates a way of writing n as sum of positive 
    integers.
    
    e.g., Number 6 can be written as
    ```
    - 6
    - 5 + 1
    - 4 + 1 + 1
    - 4 + 2
    - 3 + 1 + 1 + 1
    - 3 + 2 + 1
    - 3 + 3
    - 2 + 1 + 1 + 1 + 1
    - 2 + 2 + 1 + 1
    - 2 + 2 + 2
    - 1 + 1 + 1 + 1 + 1 + 1
    Total: 11 ways.
    ```
    Note that `p(0) = 1`
    >>> partitioning_numbers(40)
    [1, 1, 2, 3, 5, 7, 11, 15, 22, 30, 42, 56, 77, 101, 135, 176, 231, 297, 385, 490, 627, 792, 1002, 1255, 1575, 1958, 2436, 3010, 3718, 4565, 5604, 6842, 8349, 10143, 12310, 14883, 17977, 21637, 26015, 31185, 37338]
    """
    partition = [0]*(n+1)
    partition[1] = partition[0] = 1
    for x in range(2, n+1):
        delta, incr = 1, 1
        while x >= delta:
            if x >= delta:
                partition[x] = partition[x] + (partition[x - delta] if (incr & 1) else -partition[x-delta])
                delta += incr
            else:
                break
            if x >= delta:
                partition[x] = partition[x] + (partition[x - delta] if (incr & 1) else -partition[x-delta])
                delta += 2*incr + 1
                incr += 1
            else:
                break
    return partition

def shank_tonelli(n: int, m: int) -> int:
    """
    Returns the number x such that
    x = n^2 mod m
    where n, m are co-prime number
    """
    from basic_maths import gcd, mod_power
    if gcd(n, m) > 1:
        return None
    
    if mod_power(n, (m - 1) / 2, m) == m - 1:
        return None

    def factor_2_power(n):
        s = 0
        while not n & 1 and n > 1:
            n >>= 1
            s += 1
        return n, s

    q, e = factor_2_power(n)

    x, p = 2, (m - 1) / 2
    while mod_power(x, p, m) != m - 1:
        x += 1

if __name__ == "__main__":
    from doctest import testmod
    testmod()
