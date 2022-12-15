# Playground with observation
## Modular arithmetic:
Modular arithmetic modulo $n$ is defined as $a=r\ (\bmod n)$

This also can be written as $a = q\cdot p+r$, 

1. For prime modulus $p$, mod power bounces back on track in sequence when $k\geq p-1$ (A way to find a modular inverse, To multiply $a$ with $a^{-1} = a^{\phi(p)-1} = a^{p-2}$).
$$
a^k(\bmod{p}) = a^{k\pmod{(p-1)}} (\bmod p)
$$
$$
\therefore k_{\textrm{effective}} = r = k -\left\lfloor\frac{k}{p-1}\right\rfloor\cdot(p-1)
$$
Here, $r$ is remainder by dividing with $p-1$.
2. For prime modulus $p$, if $a \geq p$, 
$$
a^k\pmod{p} = (a\pmod{p})^k \pmod p
$$
3. When $p$ and $a$ are co-prime, the repetitive pattern limits to $\phi(p)$ (same as above, but the above pattern is easier to remember: when $p$ is prime, the inverse is $a^{p-2}$, if it's not prime, then evaluating [[basic_maths#Euler Totient function phi n|totient function]] is essential).

For e.g., for $a=5,\ k=9$, since $\gcd(5,\ 9)=1$, and $\phi(9)=6$, The following values will follow same mod values from $k \geq \phi(k)$.
$$
\begin{matrix}
5^0\bmod 9 & = & 1\bmod 9 & = &  1\\
5^1\bmod 9 & = & 5\bmod 9 & = &  5\\
5^2\bmod 9 & = & 25\bmod 9 & = &  7\\
5^3\bmod 9 & = & 125\bmod 9 & = &  8\\
5^4\bmod 9 & = & 625\bmod 9 & = &  4\\
5^5\bmod 9 & = & 3125\bmod 9 & = &  2\\
5^6\bmod 9 & = & 15625\bmod 9 & = &  1\\
5^7\bmod 9 & = & 78125\bmod 9 & = &  5\\
5^8\bmod 9 & = & 390625\bmod 9 & = &  7\\
5^9\bmod 9 & = & 1953125\bmod 9 & = &  8\\
5^{10}\bmod 9 & = & 9765625\bmod 9 & = &  4\\
5^{11}\bmod 9 & = & 48828125\bmod 9 & = &  2\\
5^{12}\bmod 9 & = & 244140625\bmod 9 & = &  1\\
\vdots & &\vdots & & \vdots
\end{matrix}
$$
- In this set of remainders for $9$:  $R(9)=\left\{1,\ 2,\ 4,\ 5,\ 7,\ 8\right\}$, we can observe that $\forall\ r \in R(9), \quad \gcd(r,\ 9) = 1$.  These are called [residual reduced system modulo](https://en.wikipedia.org/wiki/Reduced_residue_system).
- it's size count is the same to [[basic_maths#Euler Totient function phi n|euler's totient function]] ($\phi(9) = 6$). For $n > 2$, their sum $\sum\limits_{r\ \in\ R(n)}r$  evaluates to a multiple of $n$. In $R(9)$ case, $\sum\limits_{r\ \in\ R(9)}r = 27 = 0\ (\bmod 9)$.
4. When $p$ and $a$ are not co-prime:
[[basic_maths#Basic Math Coding problems for CS#Factors of number|Factoring]] $a$ as:
$$
a = p_1^{k_1}\cdot p_2^{k_2}\cdot p_3^{k_3}\cdots \ p_n^{k_n}
$$
$$
\mathbb{P}(n) = \left\{x: x \in \mathbb{N},\ n(\bmod p) = 0,\ F(n) = \{1, x\}\right\}
$$
$$
\mathbb{P}(a)=\left\{
	\begin{array}{ c l }
	p_1, & p_2, & p_3, & \ldots & , p_n
	\end{array}
\right\}
$$
Similarly,
$$
\mathbb{P}(p)=\left\{
	\begin{array}{ c l }
	q_1, & q_2, & q_3, & \ldots & , q_m
	\end{array}
\right\}
$$
- If $\mathbb{P}(p) \subset \mathbb{P}(a)$ (i.e., if all prime factors in $p$ are present in $a$), then after certain exponent $p\geq x$,  $a^k (\bmod p) = 0$.
- Otherwise there is no visible pattern yet.
5. [Wilson's theorem](https://en.wikipedia.org/wiki/Wilson%27s_theorem): A number $p \in \mathbb{P}\ \iff$ $(p - 1)!=-1(\bmod p)$. ($\mathbb{P}$: prime number set).
6. If $a=r (\bmod p)$, then $k^a=k^r(\bmod p)$, given $\gcd(k,\ r)=1$.

## Prime numbers
Listing prime numbers
$$
\begin{array}{ c l }
2 & 3 & 5 & 7 & 11 & 13 & 17 & 19 & 23 & 29 & 31 & 37 & 41 & 43 \\47 & 53 & 59 & 61 & 67 & 71 & 73 & 79 & 83 & 89 & 97 & 101 & 103 & 107 \\ 109 & 113 & 127 & 131 & 137 & 139 & 149 & 151 & 157 & 163 & 167 & 173 & 179 & 181 \\ 
191 & 193 & 197 & 199 & 211 & 223 & 227 & 229 & 233 & 239 & 241 & 251 & 257 & 263 \\ 269 & 271 & 277 & 281 & 283 & 293 & 307 & 311 & 313 & 317 & 331 & 337 & 347 & 349 \\ 353 & 359 & 367 & 373 & 379 & 383 & 389 & 397 & 401 & 409 & 419 & 421 & 431 & 433 \\ 439 & 443 & 449 & 457 & 461 & 463 & 467 & 479 & 487 & 491 & 499 & 503 & 509 & 521 \\ 523 & 541
\end{array}
$$
Their difference
$$
\begin{array}{ c l }0 & 1 & 1 & 3 & 1 & 3 & 1 & 3 & 5 & 1 & 5 & 3 & 1 & 3 \\5 & 5 & 1 & 5 & 3 & 1 & 5 & 3 & 5 & 7 & 3 & 1 & 3 & 1 \\3 & 13 & 3 & 5 & 1 & 9 & 1 & 5 & 5 & 3 & 5 & 5 & 1 & 9 \\1 & 3 & 1 & 11 & 11 & 3 & 1 & 3 & 5 & 1 & 9 & 5 & 5 & 5 \\1 & 5 & 3 & 1 & 9 & 13 & 3 & 1 & 3 & 13 & 5 & 9 & 1 & 3 \\5 & 7 & 5 & 5 & 3 & 5 & 7 & 3 & 7 & 9 & 1 & 9 & 1 & 5 \\3 & 5 & 7 & 3 & 1 & 3 & 11 & 7 & 3 & 7 & 3 & 5 & 11 & 1 \\17 & \end{array}
$$
Don't know the pattern yet.

## Addition.
Addition of two number are added using a certain logic gates.
For $a+b$ operation using only bitwise operator:
$$
a + b =\left\{
\begin{array}{ c l }
	a\oplus b, &\quad a\ \&\ b=0\\
	(a\oplus b) +2\cdot(a\ \&\ b), & \quad \text{otherwise recursive fn}
\end{array}
\right.
$$

This would look like a full adder logic for one bit.

|$A$|$B$|$C$|$A\oplus B\oplus C\ (\text{sum})$|$(A\wedge B)\vee(A\wedge C)\vee(B\wedge C) \text{ (carry)}$|
|-|-|-|-|-|
|0|0|0|0|0|
|0|0|1|1|0|
|0|1|0|1|0|
|0|1|1|0|1|
|1|0|0|1|0|
|1|0|1|0|1|
|1|1|0|0|1|
|1|1|1|1|1|

The carry is eventually added to next bit (i.e., multiply by $2$ or shift bits by $1$).

