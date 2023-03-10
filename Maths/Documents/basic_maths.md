# Basic Math Coding problems for CS

## Sum of first $n$ numbers.

$$\begin{array}{lll}S=1+2+3+\ldots+n  && -\ (1)\end{array}$$

Also, by commutative law $a + b = b + a$

$$\begin{array}{cl}S=n+(n-1)+(n-2)+\ldots+3+2+1 && -\ (2)\end{array}$$

Adding $1$ and $2$ we get

$$
\begin{matrix}
&S&=&1 + 2 + 3 + .... + (n - 2) + (n - 1) + n\\
+&S&=&n + (n - 1) + (n - 2) + ... + 3 + 2 + 1\\
&2\cdot S&=&\underbrace{(1 + n) + (2 + n - 1) + ... + (n + 1)}_{n \text{ terms}}
\end{matrix}
$$

$$
\therefore_ \ \ \ S = \frac{n\cdot (n+1)}{2}
$$

```python
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
```
---

## Sum of first $n$ squared numbers.

$$S=1^2+2^2+3^2+\ldots+n^2$$

Firstly, we have a identity

$$x^3-(x-1)^3=3x^2-3x+1$$

Adding the above identity from $x = 0$ to $x = n$, we get

$$n^3 = 3 \cdot (n^2 + (n-1)^2 + ... + 1) - 3\cdot\frac{n(n+1)}{2} + n $$
$$n^3 + \frac{3n(n+1)}{2} - n = 3S$$

Solving this, we get: $S = \dfrac{n(2n^2 + 2n + n + 1)}{6} = \dfrac{n(n+1)(2n+1)}{6}$.

```python
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
```
---

## Sum of first $n$ cubed numbers

The sum can be represented as

$$S=1^3+2^3+3^3+\ldots+n^3$$

Using identity:

$$n^4 - (n - 1)^4 = n^4 - (n^4 - 4n^3 + 6n^2 - 4n + 1) = 4n^3 - 6n^2 + 4n - 1 $$

Add first $n$ terms of this identity

$$\begin{array}{cl}n^4&=&4\cdot(1^3 + 2^3 + ... + n^3) - n\cdot(n+1)\cdot(2n+1) + 2n\cdot(n+1) - n\\
4\cdot S &=& n^4+n\cdot(n+1)\cdot(2n+1)-2n\cdot(n+1)+n\\
4\cdot S &=& n^4+2n^3+n^2+2n^2+n-2n^2-2n+n\end{array}
$$

The final solution

$$
\therefore\quad S=\frac{n^2(n+1)^2}{4}
$$

The solution is square of first $n$ sum $\dfrac{n(n+1)}{2}$.

---
## Sum of first $n$ triangular number
Let's consider $S_n$ as sum of first triangular numbers

$$\begin{array}{rl}S_n &=& \sum\limits_{i=1}^{n}\left(\dfrac{i\cdot(i+1)}{2}\right)\\
\implies S_n&=&1+3+6+10+\ldots +\dfrac{n(n+1)}2\\
2\cdot S_n&=&\sum\limits_{i=1}^{n}i^2+\sum\limits_{i=1}^{n}i\\
S_n&=&\dfrac{1}2\left(\dfrac{n\cdot(n+1)\cdot(2n+1)}{6}+\dfrac{n\cdot(n+1)}2\right)\\
S_n&=&\dfrac{n(n+1)(n+2)}{6}\end{array}
$$

The difference between sum of first $n$ squared and first $n$ triangular numbers:

$$\begin{array}{rll}S^2_n-S_n&=&(1-1)+(4-3)+(9-6)+(16-10)+(25-15)+\ldots +\left(n^2-\frac{(n^2+n)}{2}\right)\\
\implies S_n^2-S_n&=&S_{n-1}\end{array}$$
Another way to put it as, we fill the spaces of the lower triangle of the next term with the remaining triangle to make it a perfect square:

$$S_{n-1}+S_n=1 +\sum\limits_{i=2}^{n}\left(\frac{i\cdot(i+1)}{2}+\frac{i\cdot(i-1)}{2}\right)=1+\sum\limits_{i=2}^{n}i^2$$
## Modular power function by Binary Exponentiation

The recursive definition for this function can be defined $(\textrm{for }k \in \mathbb{N})$ as:

$$
\text{expm}(n, k, m) = 
\begin{cases}
\text{expm}\left(n,\left\lfloor\dfrac{k}{2}\right\rfloor,m\right)^2\pmod m,&k \geq 2,\ k\equiv0\pmod{2}\\
n\cdot \text{expm}\left(n,\left\lfloor\dfrac{k}{2}\right\rfloor, m\right)^{2}\pmod{m}, &k\geq2,\ k\equiv1\pmod 2\\
n,&k=1\\
1,&k=0\end{cases}
$$

An iterative example for example $2^{27}$ based on below example (without modulus):

The result is stored in $res$ and iteration is done till $k=0$

$$
\begin{matrix}
\textrm{Initialize: } & & n = 2, & res = 1, & k = 27\\
1^{st}\textrm{ Iteration: }&k\pmod2=1: & n = 4, & res = 2, & k = 13\\
2^{nd}\textrm{ Iteration: }&k\pmod2=1: & n = 16, & res = 8, & k = 6\\
3^{rd}\textrm{ Iteration: }&k\pmod2=0: & n = 256, & res = 8, & k = 3\\
4^{th}\textrm{ Iteration: }&k\pmod2=1: & n = 65536, & res = 2048, & k = 1\\
5^{th}\textrm{ Iteration: }&k\pmod2=1: & n = 4294967296, & res = 134217728, & k = 0
\end{matrix}
$$

The above step only required $\lceil\log_2{27}\rceil = 5$ steps for coming up with the answer. This is effective when we require power of the order $10^7$ or more.

```python
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
        if exponent & 1:       # equivalent to exponent % 2 == 1
            result *= number
            result %= mod
        number *= number
        number %= mod
        exponent >>= 1 # acts as floor division by 2
    return result
```
---

## Greatest Common Divisor (Also known as Highest Common Factor) of two numbers

GCD of two integers $a$ and $b$ is defined as the largest number $x$ such that 

$$
\begin{matrix}
a\equiv0\pmod x, & b\equiv0 \pmod x
\end{matrix}$$

$$
\therefore\gcd(a,b) = 
\begin{cases}
a,&b=0\\
\gcd(b,a\pmod b),&b\neq0\\
\end{cases}
$$

```python
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
```
---

## Lowest Common Multiple (LCM) of two numbers

LCM of two numbers $a$ and $b$ can be defined as:

$$\textrm{lcm}(a,\ b) = \frac{a\cdot b}{\gcd(a,b)}$$

```python
def lcm(a: int, b: int) -> int:
    """
    Calculates LCM of two integers a and b
    >>> lcm(10, 20)
    20
    >>> lcm(80, 96)
    480
    """
    return (a // gcd(a, b)) * b
```
___


## Prime Integers

We define prime number $x$ as numbers whose factors are $1$ and the number $x$ itself.

### Factors of number
Let factors of a number $n$ be:

$$
F(n) = \{x:x\in\mathbb{N},\ x\leq n,\ n\equiv0\ (\bmod{x})\}
$$

Then **prime numbers** can be defined as:

$$\mathbb{P} = \{x:\ x\in \mathbb{N},\ x \neq1,\ F(x) = \{1,\ x\}\ \}$$
The numbers that doesn't satisfy these conditions (except $n=1$) are called composite numbers.

```python
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
```

#### Composite numbers 
These are the number which have a divisor(s) other than $1$ and $n$.

$$C = \{x:\ x\in \mathbb{N}\ \textrm{and }x\notin P - \{1\}\ \}$$
Number $1$ is neither a prime or composite.

Below program finds all the factors of any values (except $1$ and $n$ itself) in $O(\sqrt{n})$ time.

```python
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
```
___

## Pascal Triangle
Pascal triangle is a triangular array of binomial coefficients. It looks something like this

$$
\begin{array}{cl}
1\\
1\quad1\\
1\quad2\quad1\\
1\quad3\quad3\quad1\\
1\quad4\quad6\quad4\quad1\\
1\quad5\quad10\quad10\quad5\quad1\\
1\quad6\quad15\quad20\quad15\quad6\quad1\\
\vdots
\end{array}
$$

Pascal triangle for $n\in\mathbb{W},r\in\mathbb{W}$ can be defined as:

$$
P(n,r) = 
\begin{cases}
1,&r\in\{0,n\}\vee n=0\\
P(n-1,r)+P(n-1,r-1),&1\leq r<n\\
0,&\textrm{otherwise}
\end{cases}
$$

```python
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
```

### Interesting:
This pattern create a fibonacci series????

$P_{00}=1$

$P_{11}+P_{20}=2$

$P_{21}+P_{30}=3$

$P_{22}+P_{31}+P_{40}=5$

$P_{32}+P_{41}+P_{50}=8$

$P_{33}+P_{42}+P_{51}+P_{60}=13$

$P_{43}+P_{52}+P_{61}+P_{70}=21$

$P_{44}+P_{53}+P_{62}+P_{71}+P_{80}=1+10+15+7+1=34$.

___

## Factorial of a number
Factorial of a number is defined as: 

$$
f(n) = 
\begin{cases}
1,&\quad n\in \{0, 1\} \\
n\cdot f(n - 1),& \quad n\in\mathbb{N}-\{0,1\}\
\end{cases}
$$

```python
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
```
___

## Fibonacci series.
The infamous series:

$$0,\ 1,\ 1,\ 2,\ 3,\ 5,\ 8,\ 13,\ 21,\ 34,\ 55,\ 89,\ 144,\ ...$$

is generated by adding two previous terms, the series starting from $0$ and $1$. It's recursive definition $fib(n)$ can be defined as:

$$
fib(n)=\begin{cases}
n, & \quad \textrm{if } n \leq 1 \\
fib(n - 1) + fib(n - 2), & \quad \textrm{otherwise}
\end{cases}
$$

```python
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
```
___

## Euler Totient function: $\phi(n)$
Euler totient function is defined as the count of natural numbers $< n$ which are co-prime to number $n$.

We represent a number as a product of primes:

$$n = p_1^{k_1}\times p_2^{k_2}\times p_3^{k_3}... \times p_n^{k_n}$$

$$\textrm{e.g., } 2520 = 2^3 \times 3^2 \times 5^1 \times 7^1$$

The euler totient function is defined as:
$$\phi(n) = n\cdot\left(1-\frac{1}{p_1}\right)\cdot \left(1-\frac{1}{p_2}\right)\cdots\left(1-\frac{1}{p_n}\right)$$
For prime numbers, $\phi(n) = n-1$; Some observations:

$$
\phi(p_1\times p_2)=\begin{cases}
\phi(p_1)\cdot\phi(p_2),&\gcd(p_1,p_2)=1\\
\phi(p_1)\cdot\phi(p_2)\cdot\dfrac{d}{\phi(d)},&d = \gcd(p_1,p_2)>1\\ 
\end{cases}
$$

```python
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
    while i * i <= n:
        if n % i == 0:
            while n % i == 0:
                n //= i
            result -= result // i
        i += 1
    if n > 1:
        result -= result // n
    return result
```
