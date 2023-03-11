# Permutations and Combinations
## Permutation
Permutation is an arrangement of elements.
For e.g., total permutation set of a set $S=\{1,2,3\}$ are $P(S)=\lbrace\lbrace1,2,3\rbrace,\lbrace1,3,2\rbrace, \lbrace2,1,3 \rbrace, \lbrace2,3,1 \rbrace, \lbrace3,1,2 \rbrace, \lbrace3,2,1 \rbrace \rbrace$.

Total permutation of $r$ objects out of $n$ objects is denoted by $P^n_r$.
In above case it's $P^3_3=6$.
Generalizing for $n$ and $r$, ${}^nP_r=\dfrac{n!}{(n-r)!}$, where $n!$ is a [[basic_maths#Factorial of a number|factorial operation]]. 

### Note:
1. Arranging $n$ items where $m_1$ are of same kind, $m_2$ are of same kind (and so on): ${}^nP_r=\dfrac{n!}{(n-r)!\cdot m_1!\cdot m_2!\cdot \cdots}$. (Here, $m_1+m_2+\cdot+m_k\leq r\leq n$). This are called multiset permutation.
2. Permutation of $n$ objects in circle: $(n-1)!$
	1. A way to summarise: in a circle, performing $n!$ will count rotations which are pointless when arranged in a circle: (e.g., in a set $A=\{1,2,3,4\}$, looking at permutations $\{1,2,3,4\},\ \{2,3,4,1\},\ \{3,4,1,2\},\ \{4,1,2,3\}$, starting from $1$ the sequence looks same). So, we'll divide total permutation by number of rotations for a particular sequence to remove counting this sequences.
		i.e., $P=\dfrac{\text{Total Permutations}}{\text{Arrangement Set Size (to remove rotations)}}=\dfrac{n!}{n}=(n-1)!$.
3. Extending this: permutation of $r$ items out of $n$ in a circle: $\dfrac{{}^nP_r}r$. 
	Another way to look at this is selecting $r$ out of $n$ objects and permutation of such $r$ objects in a circle: 
	1. Selecting $r$ objects out of $n$ objects: $\dbinom{n}{r}$ or ${}^nC_r$.
	2. Permutation of these $r$ objects in a circle: $(r-1)!$.
	Result: $(r-1)!\cdot\dbinom{n}{r}=(r-1)!\cdot\dfrac{n!}{r!\cdot(n-r)!}=\dfrac{{}^nP_r}r$.
4. Grouping objects, number of permutations where $k < r$ objects are grouped together: $P=\dfrac{(n-k+1)!}{(n-r)!}\cdot k!$
5. Permutation with repitition: selecting $n$ objects: $n^n$; selecting $r$ out of $n$ objects: $n^r$.

## Combination:
Combination is a selection of $r$ elements out of $n$ elements. Here, order of element do not matter.

For e.g., out of $4$ elements in set $A=\{1,2,3,4\}$, if we want to select $2$ items, then there are $6$ ways to do: 

$$\lbrace 1,2 \rbrace, \lbrace 1,3 \rbrace , \lbrace 1,4 \rbrace , \lbrace 2,3 \rbrace , \lbrace 2,4 \rbrace , \lbrace 3,4 \rbrace$$

We denote this by ${}^nC_r$ or $\dbinom{n}r=\dfrac{n!}{r!\cdot(n-r)!}$.

```rust
fn ncr(n: usize, r: usize) -> u128 {
    if r > n {
        0
    } else if r == 0 || r == n {
        1
    } else {
        ncr(n - 1, r - 1) + ncr(n - 1, r)
    }
}

fn ncr_memoize(n: usize, r: usize, mem: &mut Vec<Vec<u128>>) -> u128 {
    if r > n {
        0
    } else if r == 0 || r == n {
        1
    } else if mem[n][r] == 0 {
        mem[n][r] = ncr_memoize(n - 1, r, mem) + ncr_memoize(n - 1, r - 1, mem);
        mem[n][r] as u128
    } else {
        mem[n][r] as u128
    }
}

#[test]
pub fn test_ncr() {
    assert_eq!(ncr(10, 5), 252);
}

#[test]
pub fn test_ncr_memoize() {
    assert_eq!(
        ncr_memoize(50, 25, &mut vec![vec![0; 51]; 51]),
        126410606437752
    );

    assert_eq!(ncr_memoize(15, 10, &mut vec![vec![0; 51]; 51]), ncr(15, 10));
}
```

## Some results
1. In an equation: $E=(a_{11}+a_{12}+\ldots+a_{1k_1})\cdot (a_{21}+a_{22}+\ldots+a_{2k_2})\cdots(a_{m1}+a_{m2}+\ldots+a_{mk_m})$, total terms generated are: $k_1\cdot k_2\cdot k_3\cdots k_m=\prod\limits_{i=1}^mk_i$. A particular case is [[perm_and_comb#Multinomial expansion|multinomial expansion]], where a sequence of number to the power $n$ is performed.
2. An integer number defined as product of primes to a certain power: $n=p_1^{k_1}\cdot p_2^{k_2}\cdots p_m^{k_m}$: has total $\prod\limits_{i=1}^m(k_i+1)$ values that divides $n$.
## Expansion
### Binomial Expansion:
For term $(a+b)^n=B$, we'll simplify the solution:

Writing $B=\underbrace{(a+b)\cdot(a+b)\cdot(a+b)\cdot\ldots\cdot(a+b)}_{n \text{ terms}}$.
Multiplying terms $n$ times:
- There is only $1$ way to select terms such that $b$ will not occur at all, i.e., multiply all $a$'s.
- There are $n$ ways to select $b$ only once and remaining $a$'s, or which the term will be $n\cdot a^{n-1}\cdot b$.
- There are $\dfrac{n(n-1)}2$ ways to select $b$ exactly two times, and remaining $n-2$ terms $a$ which makes the term $\dfrac{n(n-1)}2\cdot a^{n-2}\cdot b^2$.
- Generalizing this: we can see there are ${}^nC_r$ ways to select $b$ $r$ times, and remaining $a$ $(n-r)$ times, which makes the term ${}^nC_r\cdot a^{n-r}\cdot b^r$.

Summing this:
$B={}^nC_0\cdot a^n+{}^nC_1\cdot a^{n-1}\cdot b^1+{}^nC_2\cdot a^{n-2}\cdot b^2+\cdots++{}^nC_n\cdot b^n$.
$B=\sum\limits_{r=0}^n{}^nC_r\cdot a^{n-r}\cdot b^r$.

We can see that there are $(n+1)$ terms in the series.
### Generator.
There are certain results of interest: e.g., ${}^{n+1}C_r={}^nC_r+{}^nC_{r-1}$.

Given that $(a+b)^n=\sum\limits_{r=0}^n{}^nC_r\cdot a^{n-r}\cdot b^r$.

Multiplying this by $(a+b)$ again:

$$
\begin{array}{ccl}
(a+b)^n\cdot(a+b)&=&\left(\sum\limits_{r=0}^n{}^nC_r\cdot a^{n-r}\cdot b^r\right)\cdot(a+b)\\
\sum\limits_{i=0}^{n+1}{}^{n+1}C_r\cdot a^{n+1-r}\cdot b^r&=&\sum\limits_{r=0}^n{}^nC_r\cdot a^{n+1-r}\cdot b^r+\sum\limits_{r=0}^n{}^nC_r\cdot a^{n-r}\cdot b^{r+1}
\end{array}
$$

Now equating terms $a^{n+1-r}\cdot b^r$ alike, we get the result ${}^{n+1}C_r={}^nC_r+{}^nC_{r-1},\quad r>0,\ n>0$.

Now, putting $a=b=1$ in expression $(a+b)^n$ we get result:

$$\sum\limits_{i=0}^n{}^nC_i=2^n$$

This also states that there are $2^n$ terms generated after multiplying terms $(a+b)$ '$n$' times. This is also used in constructing a [[basic_maths#Pascal Triangle|pascal triangle]].

### Corollary

$$
\begin{array}{cl}
(1+x)^{2n}=(1+x)^n\cdot (1+x)^n\\\\
\sum\limits_{i=0}^{2n}\dbinom{2n}{i}x^i=\left(\sum\limits_{i=0}^{n}\dbinom{n}{i}x^i\right)\cdot \left(\sum\limits_{i=0}^{n}\dbinom{n}{i}x^i\right)\\\\
\end{array}
$$

We're interested in all terms related to power $x^n$:

$$
\begin{array}{cl}
\dbinom{2n}{n}x^n=\sum\limits_{r=0}^{n}\dbinom{n}{r}\cdot \dbinom{n}{n-r}\cdot x^n\\ \\
\implies \dbinom{2n}{n}=\sum\limits_{r=0}^{n}\dbinom{n}{r}^2,&\because\dbinom{n}{r}=\dbinom{n}{n-r}
\end{array}
$$

Both of these relations can be used. The expression ${}^{2n}C_n=\sum\limits_{r=0}^n{}^nC_r\cdot {}^nC_{n-r}$ is called a catalan number, and has a lot of interpretations.
- [[memoization_1d_2#Total unique BST's https leetcode com problems unique-binary-search-trees of n nodes|Total number of binary trees that can be constructed with $n$ nodes.]].
- Total number of balancing bracket construction, where the size is $2n$.
- Total number of path that does no go above the main diagonal of the matrix (if the point starts from the bottom). Same can be said from top left to bottom right, the path does not cross the bottom of the diagonal.

## Trinomial Expansion

Trinomial expansion is exactly like binomial but with three terms, i.e., $T=(a+b+c)^n$.

Writing $T=\underbrace{(a+b+c)\cdot(a+b+c)\cdot(a+b+c)\cdot\ldots\cdot(a+b+c)}_{n \text{ terms}}$
Now:
- There is only $1$ way in which we can select $a$ term $n$ times: making the term $a^n$.
- There are $n$ ways to select values where $a$ is selected $n-1$ times, and $b$ is selected $1$ time. Additionally, $n$ ways by selecting $c$ one time: $\implies n\cdot (a^{n-1}\cdot b+a^{n-1}\cdot c)$.
- There are $\dfrac{n(n-1)}2$ to select $a$ $n-2$ times and for each of these combinations, we've to take either $b$ or $c$ or both $2$ times: $\implies \dfrac{n(n-1)}2\cdot(a^{n-2}\cdot b^2+a^{n-2}\cdot c^2+2\cdot a^{n-2}\cdot b\cdot c)$.
- There are $\dfrac{n(n-1)(n-2)}6$ to select $a$ $n-3$ times and for each of these combinations $\implies \dfrac{n(n-1)(n-2)}6\cdot(a^{n-3}\cdot b^3+3\cdot a^{n-3}\cdot b^2\cdot c+3\cdot a^{n-3}\cdot b\cdot c^2+a^{n-3}\cdot c^3)$.
- Generalizing this: we can see there are ${}^nC_r$ ways to select $a$ $r$ times, and for each of these combinations, the ways to select $p$ $b$'s and $q$ $c$'s $(q+p=n-r)$ times in total, making this expansion as 

$$
\begin{array}
\implies T&=&\dfrac{n!}{r!\cdot (n-r)!}\cdot a^r\cdot\left(\sum\limits_{m=0}^{n-r} {}^{n-r}C_m\cdot b^m\cdot c^{n-r-m}\right)\\
&=&a^{n-r}\cdot\left(\sum\limits_{m=0}^r\dfrac{n!}{r!\cdot(n-r)!}\cdot \dfrac{r!}{m!\cdot(r-m)!}\cdot b^m\cdot c^{r-m}\right)\\
&=&\sum\limits_{m=0}^{r}\dfrac{n!}{m!\cdot r!\cdot(n-r-m)!}a^{r}\cdot b^m\cdot c^{n-r-m}
\end{array}
$$

Substitute $i=r,\ j=m, k=n-r-m$, and summing all the solutions we get:

$$T=\sum\limits_{
\begin{matrix}
i,j,k\\
i+j+k=n\end{matrix}} \dfrac{n!}{i!\cdot j!\cdot k!}\cdot a^i\cdot b^j\cdot c^k$$

Putting values $a=b=c=1$, we get result: 

$$\sum\limits_{
\begin{matrix}i,j,k\\
i+j+k= n\end{matrix}} \dfrac{n!}{i!\cdot j!\cdot k!}=3^n$$

A good explaination for [generalized solution is given here](https://en.wikipedia.org/wiki/Multinomial_theorem#Number_of_ways_to_select_according_to_a_distribution).

### Total terms in the series:

Increasing power of $a$ step by step:
- For $a=0$, there are $n+1$ terms involving $b$ and $c$. (as mentioned above)
- For $a=1$, there are $n$ combination of terms involving $b$ and $c$. (as mentioned above)
- $\vdots$
- For $a=n-3$ there are $4$ combination of terms involving $b$ and $c$ as mentioned above 
- ... and so on till $a=n$.
$\therefore\quad\text{Total Terms}=1+2+3+\ldots+(n+1)=\dfrac{(n+1)(n+2)}2$.

## Pascal pyramid
This is an extension of pascal triangle into a $3$-dimensional like structure called pascal pyramid.
The generator for this pascal pyramid:

Let us consider again 

$$
(a+b+c)^n=T=\sum\limits_{
\begin{matrix}i,j,k\geq0\\
i+j+k=n\end{matrix}} \dfrac{n!}{i!\cdot j!\cdot k!}\cdot a^i\cdot b^j\cdot c^k
$$

We'll consider $i=r, j=m, k=n-r-m$ for simplicity:

Multiply by $(a+b+c)$

$$
\begin{array}{ccl}
(a+b+c)^{n+1}&=&\left(\sum\limits_{r=0}^n\dfrac{n!}{r!\cdot(n-r)!}\cdot a^{r}\cdot\left(\sum\limits_{m=0}^{n-r}\dfrac{(n-r)!}{m!\cdot(n-r-m)!}\cdot b^m\cdot c^{n-r-m}\right)\right)\cdot(a+b+c)\\
LHS&=&\left(\sum\limits_{r=0}^{n+1}{}^{n+1}C_r\cdot a^{r}\cdot\left(\sum\limits_{m=0}^{n+1-r}{}^{n+1-r}C_m\cdot b^m\cdot c^{n+1-r-m}\right)\right)\\
RHS&=&\left(\sum\limits_{r=0}^n{}^nC_r\cdot a^{r+1}\cdot\left(\sum\limits_{m=0}^{n-r}{}^{n-r}C_m\cdot b^m\cdot c^{n-r-m}\right)\right)+\\
&&\left(\sum\limits_{r=0}^n{}^nC_r\cdot a^r\cdot\left(\sum\limits_{m=0}^{n-r}{}^{n-r}C_m\cdot b^{m+1}\cdot c^{n-r-m}\right)\right)+\\
&&\left(\sum\limits_{r=0}^n{}^nC_r\cdot a^r\cdot\left(\sum\limits_{m=0}^{n-r}{}^{n-r}C_m\cdot b^m\cdot c^{n-r-m+1}\right)\right)
\end{array}
$$


Matching coefficients of $a$, $b$ and $c$:

$$
\begin{matrix}
{}^{n+1}C_r\cdot{}^{n+1-r}C_m&=&{}^nC_{r-1}\cdot{}^{n+1-r}C_m
+{}^nC_{r}\cdot {}^{n-r}C_{m-1}
+{}^nC_{r}\cdot{}^rC_m\\
\dfrac{(n+1)!}{r!\cdot  m!\cdot(n-r-m+1)!}&=&\dfrac{n!}{(r-1)!\cdot m!\cdot(n-r-m+1)!}+\dfrac{n!}{r!\cdot(m-1)!\cdot (n-r-m+1)!}+\dfrac{n!}{r!\cdot m!\cdot(n-r-m)!}\\
\dbinom{n+1}{r,m,n-r-m+1}&=&\dbinom{n}{r-1,m,n-r-m+1}+\dbinom{n}{r,m-1,n-r-m+1}+\dbinom{n}{r,m,n-r-m}
\end{matrix}
$$

Substitute $i=r,j=m,$ and $k=n-r-m$, we get

$$
\dbinom{n+1}{i,j,k+1}=\dbinom{n}{i-1,j,k+1}+\dbinom{n}{i,j-1,k+1}+\dbinom{n}{i,j,k}
$$

or we get a final expression of same nature by keeping $k$ as $k-1$:

$$
\dbinom{n+1}{i,j,k}=\dbinom{n}{i-1,j,k}+\dbinom{n}{i,j-1,k}+\dbinom{n}{i,j,k-1}
$$

Here, $0 < (i, j, k) < n$. One look at the recursive definition and the problem can be rephrased as: Find ways in a $3$-D Maze to move from top left ($(x,y,z)=(0,0,0)$) to opposite right bottom ($(x,y,z)=(m,n,p),\ m,n,p\in\mathbb{N}$), where only three moves are allowed: right ($X$ Axis), down ($Y$-Axis) or away ($Z$-Axis).

A recurrence relation for pascal pyramid is:

$$
P(n,i,j,k)=\begin{cases}
0,&i\notin[0,n],\ j\notin[0,i],\ k\notin[0,j]\\
1,&(i\leq 1)\ \vee (j\in\{0,i\}\ \wedge\ (k=j\ \vee (j=i,k=0)))\\
\begin{matrix}P(n,i-1,j,k)+P(n,i-1,j-1,k)+P(n,i-1,j-1,k-1)\end{matrix},&\text{otherwise}.
\end{cases}
$$

```cpp
#include <bits/stdc++.h>

using namespace std;
template <typename t>
using vec = vector<t>;
template <typename t>
using vec2d = vec<vec<t>>;
template <typename t>
using vec3d = vec2d<vec<t>>;

using ll = long long;

int ir(int i, int n) {
    return i < 0 || i > n;
}

int solve_pascal_pyramid(vec3d<ll>&pyramid, int n, int i, int j, int k) {
    if (ir(i, n) || ir(j, i) || ir(k, j)) {
        return 0;
    } else if (pyramid[i][j][k] != 0) {
        return pyramid[i][j][k];
    } else if ((i <= 1) || (j == 0) || (j == i && k == 0) || (j == i && k == j)) {
        /// Use case: it's any one corner vertex of a tertrahedron
        pyramid[i][j][k] = 1;
        return pyramid[i][j][k];
    } else {
        pyramid[i][j][k] = solve_pascal_pyramid(pyramid, n, i-1, j, k) + solve_pascal_pyramid(pyramid, n, i-1, j-1, k) + solve_pascal_pyramid(pyramid, n, i-1, j-1, k-1);
        return pyramid[i][j][k];
    }
}

int main () {
    int n = 20;
    vec3d<ll>pyramid(n, vec2d<ll>(n, vec<ll>(n, 0)));
    for (int i = 0; i < n; ++i) {
        for (int j = 0; j <= i; ++j) {
            for (int k = 0; k <= j; ++k) {
                solve_pascal_pyramid(pyramid, i, i, j, k);
            }
        }
    }
    for (int i = 0; i < n; ++i) {
        for (int j = 0; j <= i; ++j) {
            for (int k = 0; k <= j; ++k) {
                cout << pyramid[i][j][k] << " ";
            }
            cout << endl;
        }
        cout << endl;
    }
    return 0;
}
```

Another sample code is similar to the final expression: where a user is trapped in a 3D maze:

```cpp
#include <bits/stdc++.h>

using namespace std;
using ll = long long;

int main () {
    int n = 10;
    int a[n][n][n];
    for (int i = 0; i < n; ++i) {
        for (int j = 0; j < n; ++j) {
            for (int k = 0; k < n; ++k) {
                a[i][j][k] = 0;
            }
        }
    }
    a[0][0][0] = 1;
    for (int i = 0; i < n; ++i) {
        a[i][0][0] = a[0][i][0] = a[0][0][i] = 1;
    }
    for (int i = 0; i < n; ++i) {
        for (int j = 0; j < n; ++j) {
            for (int k = 0; k < n; ++k) {
                if (!a[i][j][k]) {
                    a[i][j][k] = (i > 0 ? a[i-1][j][k] : 0) + (j > 0 ? a[i][j-1][k] : 0) + (k > 0 ? a[i][j][k-1] : 0);
                }
            }    
        }
    }
    for (int i = 0; i < 3; ++i) {
        for (int j = 0; j < 3; ++j) {
            for (int k = 0; k < 3; ++k) {
                cout << a[i][j][k] << " ";
            }
            cout << endl;
        }
        cout << endl;
    }
}
```

# Multinomial expansion:
Generalizing above value, we've

$$
\begin{array}{cl}
(x_1+x_2+\ldots+x_m)^n=E&=&\sum\limits_{\begin{matrix}k_1,k_2,\ldots,k_m>0\\ 
k_1+k_2+\cdots+k_m=n\end{matrix}}\dfrac{n!}{k_1!\cdot k_2!\cdot k_3!\ldots k_m!}\cdot\prod\limits_{i=0}^m x_i^{k_i}\\
&=&\sum\limits_{\begin{matrix}k_1,k_2,\ldots,k_m>0\\ 
k_1+k_2+\cdots+k_m=n\end{matrix}}\dbinom{n}{k_1,k_2,\ldots k_m}\cdot\prod\limits_{i=0}^m x_i^{k_i}
\end{array}
$$

There is a generalized relation for $m$-dimensional pyramid (note: $\sum\limits_{i=1}^mk_i=n$):

$$
\dbinom{n+1}{k_1,k_2,\ldots,k_m}=\dbinom{n}{k_1-1,k_2,\ldots,k_m}+\dbinom{n}{k_1,k_2-1,\ldots,k_m}+\cdots+\dbinom{n}{k_1,k_2,\ldots,k_m-1}
$$

**Proof**: performing operation on $RHS$:

$$
\begin{array}{cll}
RHS&=&\dfrac{n!}{\prod\limits_{i=1}^m(k_i-1)!}\cdot\left(\dfrac{1}{k_2\cdot k_3\cdots k_m}+\dfrac{1}{k_1\cdot k_3\cdots k_m}+\cdots+\dfrac{1}{k_1\cdot k_2\cdots k_{m-1}}\right)\\
&=&\dfrac{n!}{\prod\limits_{i=1}^m(k_i-1)!}\cdot\left(\dfrac{k_1}{k_1\cdot k_2\cdots k_m}+\dfrac{k_2}{k_1\cdot k_2\cdots k_m}+\cdots+\dfrac{k_m}{k_1\cdot k_2\cdots k_m}\right)\\
&=&\dfrac{n!}{\prod\limits_{i=1}^m k_i!}\cdot\left(\sum\limits_{i=1}^m k_i\right)\\
&=&\dfrac{(n+1)\cdot n!}{\prod\limits_{i=1}^m k_i!}\quad(\text{as per LHS, }\sum\limits_{i=1}^mk_i=(n+1))\\
&=&\dfrac{(n+1)!}{\prod\limits_{i=1}^m k_i!}=LHS
\end{array}
$$

Similarly, a recurrence relation of $K=\{k_1,k_2,\ldots,k_m\}$will be:

$$
P(n,k_1,k_2,\ldots,k_m)=\begin{cases}
1,&(\exists!\ i\in[1,m],\ k_i=0)\ \vee (\forall\ i\in[1,m],\ k_{i}=k_1) \text{ (To be checked)}\\
0,&k_1\notin[0,n], \forall\ i\in[2,m],\ k_i\notin[0,k_{i-1}]\\
\sum\limits_{i=1}^mP(n,k_1',k_2',\ldots,k_{i-1}',k_i',k_{i+1},\ldots,k_m),&\forall\ l\in[1,i],\ k_l'
=k_l-1\end{cases}
$$

The first expression simply states that value is $1$ if it's one of the corner vertices of a $n$-dimensional tetrahedron.

The last expression stands for simply:

$$
P(n,k_1-1,k_2,\ldots,k_m)+P(n,k_1-1,k_2-1,\ldots,k_m)+P(n,k_1-1,k_2-1,k_3-1,\ldots,k_m)+\cdots+P(n,k_1-1,k_2-1,\ldots,k_m-1)
$$

# Derangement of a permutation.
Given a set $A$, we're finding total arrangement where none of the values are in their original position.

For e.g., $A=\{1,2,3,4\}$, there are $9$ derangements: 

$$\lbrace 2,3,4,1 \rbrace , \lbrace 3,4,1,2 \rbrace , \lbrace 4,1,2,3 \rbrace , \lbrace 3,4,2,1 \rbrace , \lbrace 4,3,1,2 \rbrace , \lbrace 3,1,4,2 \rbrace , \lbrace 2,4,1,3 \rbrace , \lbrace 4,3,2,1 \rbrace, \lbrace 2,1,4,3 \rbrace$$

Note that none of the values in these arrangements are in same position as in set $A$. Number of such arrangements $!n$ can be evaluated step by step:

### Expression: evaluate $D(A)=!n$.
Assume that size of set $A=|A|=n$.

- Permutation of set $A=n!\implies D(A)=!n=n!$ (assume initially).
- Considering that we counted permutations with at least one in place; exclude these values from the permutations: $\dbinom{n}{1}\cdot(n-1)!=\dfrac{n!}{1!}\implies D(A)=n!-\dfrac{n!}{1!}$. 
- We again have to count the permutations where at least two values are in place; so that we can include these value in the result: $\dbinom{n}{2}\cdot(n-2)!=\dfrac{n!}{2!}\implies D(A)=n!-\dfrac{n!}{1!}+\dfrac{n!}{2!}$
	- Explanation: We've ${}^nC_2$ ways of selecting positions where at least two values are in same place, and remaining $(n-2)$ values will be permuted: $\dfrac{n!}{2!\cdot (n-2)!}\cdot (n-2)!=\dfrac{n!}{2!}$. Note that they might be more values in place, but this expression keeps certain that there are **at least two in place**.
- Generalizing this for any $r\leq n$, depending on the previous operation (inclusion/exclusion), we perform the operation so that $D(A)=!n=\sum\limits_{r=0}^n(-1)^r\cdot \dfrac{n!}{r!}$.

At the point where $r=n$, we've evaluated the derangement of the values. 
The expression:

$$
!n=n!\cdot\left(\dfrac{1}{0!}-\dfrac{1}{1!}+\dfrac{1}{2!}-\dfrac{1}{3!}+\dfrac{1}{4!}+\cdots+(-1)^n\cdot\dfrac{1}{n!}\right)
$$

These are the first $n$ terms of [[intermediate_maths#Taylor series for e x|taylor series]] of $e^x$ with $x=-1$. With this, we can approximate the value $!n\approx \dfrac{n!}{e}$. 

Also, [[probability]] that none of the permutations are in place will be:

$$
P(A)=\dfrac{|!n|}{|n!|}=\dfrac{\left(\dfrac{n!}{e}\right)}{n!}=\dfrac1e
$$

This happens when $n$ is too big of a number or $n\rightarrow \infty$.
A good explaination is also given on [wikipedia](https://en.wikipedia.org/wiki/Derangement#Derivation_by_inclusion%E2%80%93exclusion_principle).

```rust
fn derangement(n: usize) -> u128 {
    let mut result: i128 = 0;
    let mut mul = 1;
    let mut sign: i128 = if n & 1 == 0 { 1 } else { -1 };
    for x in ((0 as i128)..=(n as i128)).rev() {
        result += sign * mul;
        sign = -sign;
        mul *= x;
    }
    result as u128
}

#[test]
pub fn test_derangement() {
    assert_eq!(derangement(9), 133496);
    assert_eq!(derangement(8), 14833);
    assert_eq!(derangement(10), 1334961);
}
```