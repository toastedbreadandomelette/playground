# Intermediate Maths for CS: Part 1
## [Taylor series](https://en.wikipedia.org/wiki/Taylor_series)
## [Trigonometric series](https://en.wikipedia.org/wiki/Trigonometric_functions)
All equation of a floating number $x$, where $x$ is an angle in radian.
### Sine: $\textrm{sin}(x)$

For an angle $x$, $\sin(x) = \dfrac{\textrm{opposite side to the angle}}{\textrm{hypotenuse}} = \dfrac{a}{\sqrt{a^2+b^2}}$

$$\begin{array}{rll}\sin(x)&=&\sum\limits_{n=0}^{\infty}\dfrac{(-1)^nx^{2n+1}}{(2n+1)!}\\
\sin(x)&=&x-\dfrac{x^3}{3!}+\dfrac{x^5}{5!}-\dfrac{x^7}{7!} +\cdots\end{array}
$$

```desmos-graph
y=\sin(x)
```

### Cosine: $\textrm{cos}(x)$

For an angle $x$, $\cos(x) = \dfrac{\textrm{adjacent side to the angle}}{\textrm{hypotenuse}} = \dfrac{b}{\sqrt{a^2+b^2}}$

$$
\begin{array}{rll}\cos(x)&=&\sum\limits_{n=0}^{\infty}\dfrac{(-1)^nx^{2n}}{(2n)!}\\
\cos(x) &=& 1-\dfrac{x^2}{2!}+\dfrac{x^4}{4!}-\dfrac{x^6}{6!}+\cdots\end{array}
$$

```desmos-graph
y=\cos(x)
```
### Tangent: $\textrm{tan}(x)$

$$
\textrm{tan}(x) = \frac{\textrm{sin}(x)}{\textrm{cos}(x)}
$$

```desmos-graph
y=\tan(x)
```
### Cotangent: $\textrm{cot}(x)$

$$
\textrm{cot}(x) = \frac{1}{\textrm{tan}(x)} = \frac{\textrm{cos}(x)}{\textrm{sin}(x)}
$$

```desmos-graph
y=\cot(x)
```
### Secant: $\textrm{sec}(x)$

$$
\textrm{sec}(x) = \frac{1}{\textrm{cos}(x)}
$$

```desmos-graph
y=\sec(x)
```

### Cosecant: $\textrm{cosec}(x)$ or $\textrm{csc}(x)$

$$
\textrm{csc}(x) = \frac{1}{\textrm{sin}(x)}
$$

```desmos-graph
y=\csc(x)
```
___
### Program for all trigonometric functions.
```python
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

```
___
### [Inverse trigonometric functions](https://en.wikipedia.org/wiki/Inverse_trigonometric_functions)
#### Sine inverse: $\textrm{arcsin}(x)$

$$
\textrm{arcsin}(x) = \sum\limits_{n=0}^{\infty}\frac{(2n)!}{(2^n.n!)^2}\cdot\frac{z^{2n+1}}{2n+1},\ \ |x| < 1
$$

This series is evaluated and expanded as:

$$
\textrm{arcsin}(x) = x + \left(\frac{1}{2}\right)\cdot\frac{z^3}{3}+\left(\frac{1 \cdot 3}{2 \cdot 4}\right)\cdot\frac{z^5}{5} + \left(\frac{1 \cdot 3 \cdot 5}{2 \cdot 4 \cdot 6}\right)\cdot\frac{z^7}{7} + \cdots
$$

```python
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
```
#### Cos inverse: $\textrm{arccos}(x)$

$$
\textrm{arccos}(x) = \frac{\pi}{2} - \textrm{arcsin}(x)
$$

```python
def arccosine(x: float) -> float:
    """
    Compute inverse cosine
    """
    return pi / 2 - arcsine(x)
```

#### Tan inverse: $\textrm{arctan}(x)$
Using Euler's convergence:

$$
\textrm{arctan}(x) = \sum\limits_{n=0}^{\infty}\frac{2^{2n}(n!)^2}{(2n+1)!}\cdot\frac{x^{2n+1}}{(1+x^2)^{n+1}}
$$

```python
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
```
___
### Taylor series for $e^x$

$$
e^x = \sum\limits_{n=0}^{\infty}\frac{x^n}{n!}
$$

$$
e^x = 1 + \frac{x}{1!} + \frac{x^2}{2!} + \frac{x^3}{3!} + \frac{x^4}{4!} + \cdots
$$

```desmos-graph
top=150; bottom=-20;
left=-10; right=10;
---
y=e^x
```

```python
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
```
___
### Logarithm: $\ln(x)$

$$
\ln(x) = \sum\limits_{n=0}^{\infty}\frac{1}{2n+1}\left(\frac{x-1}{x+1}\right)^{n},\ \ |x| < 1
$$

```desmos-graph
y=\ln(x)
```

```python
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
```
___
## Square root of a number: $\sqrt{n}$
The method to find square root is by newton's method. Let us assume for function $f(n) = \sqrt{n}$, if the function satisfies the condition, then:

$$
x_n = x_{n-1} - \frac{f(x_{n-1})}{f'(x_{n-1})}
$$

would be a better approximate value than $x_{n-1}$. The above equation is derived by solving for $x_n$:

$$
f'(x_{n-1}) = \frac{f(x_{n-1}) - 0}{x_{n-1} - x_{n}}
$$
Starting from $x_0$ (which can be an arbitrary value, but the closer to zero, the better), we evaluate the method till certain condition is satisfied (the below solution keeps the precision check upto $10^{-18}$).

For solving $x^2 - n = 0$,

$$x_{n+1} = x_{n} - \frac{x_n^2-n}{2x_n}$$

$$x_{n+1} = \frac{1}{2} \left(x_n + \frac{n}{x_n} \right)$$
The convergence is very high for this method.

```python
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
```

An integer square root can be defined as

$$
\textrm{isqrt}(n) = \lfloor\sqrt{n}\rfloor
$$

In this case, the algorithm can be terminated when value of $x_n = 0.$

```python
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
```

This technique is also used for dividing two numbers.

## Dividing two numbers.

## Sieve of Eratosthenes
Sieve of eratosthenes is a special method of marking [[basic_maths#Composite numbers|composite numbers]], leaving prime values unmarked: these values are then used to mark their multiples as composites. [A useful demonstration of this method can be found here](https://en.wikipedia.org/wiki/File:Sieve_of_Eratosthenes_animation.gif).

```python
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
```

Note that the array is pre-computed upto $n$ positive integers, so after computing for $n$ values and storing in array, primality check becomes $O(1)$. 

The overall complexity of pre-computing using this method is $O(n\log(\log\sqrt{n}))$. A good proof of the complexity is given [here](https://cp-algorithms.com/algebra/sieve-of-eratosthenes.html#asymptotic-analysis). (The proof shows $O(n\log(\log{n}))$ but the loop 
```python 
for number in range(3, sqrt_n + 1):
```
 shortened it from $n$ to $\sqrt{n}$).

Optimizations:
	1. Instead of using all indices, checking and marking for odd numbers would reduce time and space complexity by half.
	2. Marking bits instead of integer/boolean values (in some languages, takes $1$ byte of size) with the above optimization would save a lot of space (around $16\times$ the space currently required).
___
## Segmented Sieve.
[[#Sieve of Eratosthenes]] is good when the values are in range of around $10^7$. 

For finding prime values between range $a$ and $b$ (given that the difference is the same mentioned above), and $b$ is in the range $\leq 10^{12}$, segmented sieve does the job. (note that $b-a<10^7$)

**Algorithm**:
- For a given value $b$, Evaluate all prime values from $2$ to $\sqrt{b}$: let this be stored in set $P$
- For each prime $p$ in $P$:
- Find starting value $x$  ($b \geq x \geq a$). If $p^2$ is not a starting value, then find one such that: $x\mod{p} = 0$

$$
x = \max\left(p^2,\ p.\left\lceil\frac{a}{p}\right\rceil\right)
$$

- From $x$ till $b$, by skipping $p$ numbers, mark values as true (say $M$ as marker).

$$
M_{x-a} = \textrm{false}
$$

```python
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
```

Now, to test whether number $n$ is prime or not, can be checked whether $M_{n - a}$ is true or false.


