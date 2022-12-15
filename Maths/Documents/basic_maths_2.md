# Basic Maths 2
Link for [[basic_maths|basic maths part 1]].

## Sum of first $k$-gonal numbers:

You can find it [here](https://en.wikipedia.org/wiki/Polygonal_number)
$$
P^k_n=1+\sum\limits_{i=1}^{n-1}((k-2)\cdot i+1)
$$
$$
\implies P_n^k=n+(k-2)\frac{n(n-1)}{2}
$$
For $n=4$, this turns out to be:
$$
P_n^4=n+n^2-n=n^2
$$
## Sum of first $k$-gonal pyramid

$$
S(P_n^k)=\sum\limits_{i=1}^{n}\frac{2i+(k-2)(i^2-i)}{2}
$$
Not gonna solve this bro. I'm bored.

## Arithmetic series:
### Finding $n^{th}$ term:
Consider $t_n$ $n^{th}$ term, given first term $a$ (or $t_1$) and difference $d$, then:
$$
t_n=a+(n-1)\cdot d
$$
Consecutive difference can be found from any two terms $L$ and $R$ as:
$$
d=\left(\frac{t_R-t_L}{R-L}\right),\quad L\neq R
$$
- Sum of first $n$ terms of such series.
$$
\begin{array}{cl}
S_n&=\sum\limits_{i=1}^{n}(a+(i-1)\cdot d)\\
&=n\cdot a+\left(\dfrac{n(n+1)}{2}-n\right)\cdot d\\
&=\dfrac{n}{2}\cdot(2a+(n-1)\cdot d)
\end{array}
$$
### Arithmetic mean of a number/series
Arithmetic mean is defined as an average of all the number.
Consider set $A=\{a_1,\ a_2,\ a_3,\ \ldots,\ a_n\}$.
i.e., $AM(A)=\dfrac{\sum\limits_{i=1}^n a_i}{n}=\dfrac{a_1+a_2+a_3+\ldots+a_n}{n}$.

If consecutive pairs are equidistant (i.e., they are in Arithmetic progression with $a_i$ being the $i^{th}$ term), then average is:
$$
AM(A)=\left\{
\begin{array}{cl}
a_k,&k=\left\lceil\dfrac{n}{2}\right\rceil,\ n\equiv1\pmod{2}\\
\dfrac{a_k+a_{k+1}}{2},&k=\dfrac{n}{2},\ n\equiv0\pmod{2}\\
\end{array}
\right.
$$

- Arithmetic point tells us the center of distribution of uniform points: in a line it defines a middle average of all the points.
- If these values are weighted, (e.g., for value $a_i$, the weight is $m_i$), then the average of all of them is:
$$
AM(A,M)=\dfrac{\sum\limits_{i=1}^{n}a_i\cdot m_i}{\sum\limits_{i=1}^{n}m_i}=\dfrac{a_1\cdot m_1+a_2\cdot m_2+\ldots+a_n\cdot m_n}{m_1+m_2+\ldots +m_n}
$$
# Geometric Series
The $n^{th}$ term with the starting value of series as $a$ and factor as $r$ is:
$$
t_n=a\cdot r^{n-1}
$$
Given $t_L$ and $t_R$ ($L<R$), it's factor $r$ can be evaluated as:
$$
r=\sqrt[(R-L)]{\dfrac{t_R}{t_L}}
$$
(Read as $(R-L)^{th}$ root of ratio of $t_R$ and $t_L$).
### Summation
The sum of series:
$$
S_n=a+a\cdot r+a\cdot r^2+\ldots+a\cdot r^{n-1}=\sum\limits_{i=0}^{n-1}a\cdot r^i
$$
$$
r\cdot S_n-S_n=a\cdot (r^{n}-1)
$$
$$
\implies S_n=\dfrac{a\cdot (r^n-1)}{r-1}
$$
when $r<1$, and $n\rightarrow\infty$, then
$$
S_{\infty}=\frac{a}{1-r}
$$
i.e., the infinite series converges to a certain value.
## Geometric Mean
Geometric mean on set $A$ is defined as
$$
GM(A)=\sqrt[n]{\prod\limits_{i=1}^{n}a_i}=\sqrt[n]{a_1\cdot a_2\ldots\cdot a_n}
$$

## Chinese Remainder Theorem (CRT)
We're given system of linear equations:
$$
\begin{matrix}
a\equiv a_1\pmod{p_1}\\
a\equiv a_2\pmod{p_2}\\
\vdots\\
a\equiv a_k\pmod{p_k}\\
\end{matrix}
$$
where $\forall\ i,j\in[1,k],i<j,\gcd(p_i,p_j)=1$. (Every pair are co-prime).
It states that given set of congruence always has *one and exactly one* solution.
We'll use Garner's mixed radix representation to denote number $a$.

$$
a=x_1+x_2\cdot p_1+x_3\cdot p_1\cdot p_2+\ldots+x_k\cdot p_1\cdot p_2\ldots\cdot p_{k-1}
$$
We're required to evaluate values $x_i,i\in[1,k]$.

Let $r_{ij}\equiv p_i\pmod{p_j}$. 

And we have from above equation:
$$
a_1\equiv x_1\pmod{p_1}
$$
We can evaluate $a_2$ as:
$$
\begin{matrix}
a_2&\equiv & x_2\pmod{p_2}\equiv (a_1+x_2\cdot p_1)\pmod{p_2}\\
(a_2-x_1)&\equiv&x_2\cdot p_1\pmod{p_2}\\
(a_2-x_1)\cdot r_{12}&\equiv&x_2\pmod{p_2}&-\ (\text{multiply by }\ p_1^{-1}=r_{12})
\end{matrix}
$$
Similarly:
$$
((a_3-x_1)\cdot r_{13}-x_2)\cdot r_{23}\equiv x_3\pmod{p_3}
$$

With this pattern we can evaluate all the values of $x$ and construct $a$.