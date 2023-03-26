# Memoization
Memoization is remembering things to use it in future, with certain values depending on certain condition.

We generally use [[arrays_1d|arrays]] or hash-maps to store the calculated results for next bigger problem.
To solve such problems we need to identify what are the possible next steps from $f(n)$ to $f(n\pm k)$ to perform: depending on the problem statement, with valid base cases.

# One Dimensional

## Fibonacci Series
[[basic_maths#Fibonacci series|Fibonacci series]] uses a recurrence relation:

$$
fib(n)=\begin{cases}
n, & n \leq 1\\
fib(n-1) + fib(n-2), & n>1
\end{cases}
$$

Here, $n\in\mathbb{N}$. 
We're certain the $n^{th}$ term determines the value with certain base case, so we can create a $memo$ (of size $n$ for array or just a hashmap), and store the computed value in $memo$ under key $n$.
i.e., 

$$
memo_n = fib(n)=\begin{cases}
n, & n \leq 1\\
memo_n, & n\in memo\\
fib(n-1) + fib(n-2), & n\notin memo,\ n>1
\end{cases}
$$

This computes and stores the results of $memo[n-1], memo[n-2], \ldots, memo[3], memo[2]$: and are used twice (once during computation recursive computation, and once during direct returning from $memo$).
This is a **Top-Down approach**.

These can also be constructed as:

$$
\begin{array}{cl}
memo_0=0\\
memo_1=1\\
memo_2=memo_1+memo_0\\
\vdots\\
memo_n=memo_{n-1}+memo_{n-2}
\end{array}
$$

This kind of building solution is a **Bottom-up iterative approach**.
Extending this to any recursive function:

$$
memo_n=f(n)=\begin{cases}
c_0, & n = 0\\
c_1, & n = 1\\
\vdots\\
c_k, & n = k\\
memo_n, & n\in memo\\
a_0\cdot f(n-1)^{p_0}+a_1\cdot f(n-2)^{p_1}+\cdots+ a_{k}\cdot f(n-k-1)^{p_k}, & n\notin memo, n>k
\end{cases}
$$

```ad-note
The recurrence relation we chose are simple and contains consecutive $n$ terms. The ones you see might not be a [[intermediate_maths_2#Linear Recurrence|linear one]], and the evaluating $n^{th}$ term might not be dependent on immediate terms as well (e.g., [[memoization_1d_1#Collatz Sequence|Collatz Sequence]]).
```

## Some recurrence relations with one variable:

- Tribonacci

$$f(n)=\begin{cases}
n, & n\leq 1\\
1, & n=2\\
f(n-1)+f(n-2)+f(n-3), & n>2
\end{cases}
$$

- Number of ways/paths to climb the stairs: A guy on $k^{th}$ step can take either skip one and go to next (landing on $(k+2)^{th}$ step) or just climb (landing on $(k+1)^{th}$ step). (Here, $f(0)=1, f(1)=1$). This is similar to fibonacci, except that base condition is changed (skipped by $1$).
	- Why $f(0)=1$?: The number of ways to climb the stairs concerns about the total paths from $0\rightarrow n$ (for e.g., for $f(2)$, there are two paths that can be taken: $0\rightarrow1\rightarrow2$ and $0\rightarrow2$). But for $0$, the path is $0$, which is still a valid path.

$$
f(n)=\begin{cases}
1, & n\leq 1\\
f(n-1)+f(n-2), & n>1
\end{cases}
$$

- The above problem looks attached to the real world, it's mathematically number of ways (different arrangements are considered different) you can add numbers upto $n$ using only $1$ and $2$. (for tribonacci, it's $1$, $2$ and $3$).
e.g., for Fibonacci: $f(5)=8$ has following solution:

$$
\begin{array}{cl}
5=1+1+1+1+1\\
=1+1+1+2\\
=1+1+2+1\\
=1+2+1+1\\
=2+1+1+1\\
=1+2+2\\
=2+1+2\\
=2+2+1
\end{array}
$$

- [Tiling Problem](https://www.geeksforgeeks.org/tiling-problem/) has also the same solution
- E.g., similar to above value, similar problem [Dice Combinations](https://cses.fi/problemset/task/1633) deals with the similar problem, only difference is using values from $1\rightarrow6$.
- An iterative system can be also be created building from base case, considering $f_n$ as the $n^{th}$ term:

$$
\begin{array}{cl}
f_0=c_0\\
f_1=c_1\\
\vdots\\
f_k=c_k\\
\forall\ i: k+1\rightarrow n,\quad f_i=a_0\cdot f_{i-1}^{p_0}+a_1\cdot f_{i-2}^{p_1}+\ldots +a_k.f_{i-k-1}^{p_k}
\end{array}
$$

The memo $f$ is further to return value in $O(1)$ time whenever user calls the function $f(n)$, second time.

```python
def dice_combinations(n: int) -> int:
    """
    >>> dice_combinations(50)
    660641036
    >>> dice_combinations(1000)
    937196411
    >>> dice_combinations(654321)
    615247550
    """
    if n <= 6:
        return (1 << (n - 1))
    memo = [0] + [(1 << (n - 1)) for n in range(1, 7)]
    for q in range(7, n + 1):
        memo.append((memo[-1] + memo[-2] + memo[-3] +
                    memo[-4] + memo[-5] + memo[-6]) % 1000000007)
    return memo[-1]
```

## Collatz Sequence

Collatz conjecture (say $f(n)$) defines function as

$$
f(n)=\begin{cases}
\dfrac{n}{2}, & n\equiv0 \pmod{2}\\
3n+1, & n\equiv1\pmod{2}
\end{cases}
$$
One can also define this in new way:

$$
f(n)=\begin{cases}
\dfrac{n}{2}, & n\equiv0 \pmod{2}\\
\dfrac{3n+1}{2}, & n\equiv1\pmod{2}
\end{cases}
$$

To skip the odd part, since $\forall\ n\in\mathbb{N},\ 3n+1\equiv0\pmod{2}$.

We evaluate the steps taken to reduce from $n$ to $1$ using this function, and storing the count against $n$ (let's call it $f(n)$).

$$
f(n)=\begin{cases}
0,&n=1\\
1+f\left(\dfrac{n}{2}\right),&n\equiv 0\pmod2\\
2+f\left(\dfrac{3n+1}{2}\right),&n\equiv 1\pmod2\\
\end{cases}
$$

i.e., For $n=8$, $memo[8]=3,\because f(f(f(8)))=f(f(4))=f(2)=1$.

Some corollaries:
- For $n=2^k$, the value of $f(n)=k$.
- From the above results, if $n\equiv1\pmod{2}$ and $3n+1=2^k$, then n would look like an alternating values of $1$'s and $0$'s. (why does it matter?)

$$
\begin{array}{cl}
5_{10}=101_2\implies3\cdot5+1=16\implies f(5)=1+f(16)(=4)=5\\
21_{10}=10101_2\implies3\cdot21+1=64\implies f(21)=1+f(64)(=6)=7\\
85_{10}=1010101_2\implies3\cdot85+1=256\implies f(85)=1+f(256)(=8)=9\\
\end{array}
$$

## House Robbers
House robbers deals with maximizing value a robber can steal from sealed houses, given that they cannot steal from two adjacent houses.

So the total cost from the starting till the $i^{th}$ current house can be calculated as:
1. Either robber should leave the current house and move to next: in the case the cost $cst(i)$ is the same as in the previous house $cst(v,\ i-1)$.
2. We did not steal from previous neighbor, so we steal current and see how does it add up $val[i]+cst(v,\ i-2)$.
3. One of these two combination that would give the maximum value is the answer.
i.e., 

$$
cst(v,\ i)=\begin{cases}
0, & i<0\\
v[i], & i = 0\\
\max(cst(v,i-1),\ v[i]+cst(v,i-2)), & i>1
\end{cases}
$$

```python
def rob(nums: list) -> int:
    """
    Determines the max cost that robber can steal from neighborhood.
    >>> rob([1, 2, 3, 1])
    4
    >>> rob([2, 1, 1, 2])
    4
    >>> rob([3, 1, 1, 1, 5, 1, 6])
    15
    """
    def robbing(memo, nums, pos):
        if pos < 0:
            return 0
        elif pos == 0:
            memo[0] = nums[0]
        elif pos not in memo:
            memo[pos] = max(robbing(memo, nums, pos-1), nums[pos] + robbing(memo, nums, pos-2))
        return memo[pos]
    return robbing({}, nums, len(nums)-1)
```

### Minimum amount of Coins.

A recursive approach for coin change of value $n$ from available coin set $w$ (say $cc(w,n,i)$), we can either:
- Include the $i^{th}$ coin in the coin change (subtract: $n-w[i]$), and add it to the result $\implies cc(w,n-w[i],i) = r_1$ (say)
- Select next coin ($i+1$).$\implies cc(w,n,i+1)=r_0$ (say)
i.e., 

$$
cc(w,n,i)=\begin{cases}
0, & n=0\\
-1, & i>|w|\ \|\ n<0\\
\min(1+r_1,r_0), &r_1\geq0,\ r_0\geq0\\
1+r_1,&r_0=-1,r_1\geq0\\
r_0,&r_1=-1,r_0\geq0
\end{cases}
$$

**Note that** this approach assumes that you have infinite amount of reserve of coins, using them as many times as you want.

A memiozation approach:
The memoization approach can be made as follows:

$$
memo_n=cc(w,n)=\begin{cases}
0,&n=0\\
-1,&n<0\vee(\forall\ k\in[0,|w|), cc(w,n-w[k])=-1) \\
\min\limits_{k=0,\ cc(w,n-w[k])\neq-1}^{|w|}(1+cc(w,n-w[k])),&n>0,cc(w,n-w[k])\neq 1
\end{cases}
$$

## Subset sum with divisibility

From given array $A$, find the subset $S$ such that $|S|\equiv0\bmod{p}$.

If $|A|\geq p$, then there is always a solution, which is a subsegment/subarray:
- For any solution, $|S|\equiv a\bmod{p}\implies0\leq a\leq p-1$. (There are $p$ unique remainders)
- let $b_k=\sum\limits_{i=1}^{k}A_i$: then
$b_0=0$
$b_1=A_1$
$b_2=A_1+A_2$
$\vdots$
$b_n=A_1+A_2+\ldots+A_n$
- Now, if there is at least two prefix sums of equal modulo (say values $b_L$ and $b_R$) then $\sum\limits_{i=L+1}^{R}A_i=b_R-b_L\equiv 0\bmod{p}$: i.e., it's for sure that subarray is divisible by $p$. 
- This is guaranteed if $n\geq p$. (i.e., there are $n+1$ $b_k$ values: which are more than all the possible values of $p$ when $n\geq p$). hence at least $n+1-p$ values will coincide with some of the mod values.
$\therefore$ The function (say $g$) for:
	- $A$: Set of integers
	- $s$: current sum of selected items
	- $p$: modulo value
	- $i$: current index of a set.
is determined by two ways:
1. Either to include the value $A_i$ in the sum $s$, ($s+A_i$) and move to next value ($i+1$)
2. Skip the value $A_i$ ($i+1$). 
If either of the value returns true, then there exists such subset.

$$
g(A,s,p,i)=\begin{cases}
\text{true},&|A|\geq p\ \vee (s\equiv0\bmod{p},\ s\neq0)\\
\text{false},&i\geq|A|,s\not\equiv 0\bmod{p}\\
g(A,s+A_i,p,i+1)\vee g(A,s,p,i+1),&\text{otherwise}
\end{cases}
$$

This can be stored in a $memo$ that would take $O(p)$ space.

```python
def subset_sum_divisibility(array: list, m: int) -> bool:
    """
    Check whether there is a subset with sum divisible by zero.
    >>> subset_sum_divisibility([3, 1, 7, 5], 6)
    True
    >>> subset_sum_divisibility([3, 1, 7, 4], 6)
    True
    >>> subset_sum_divisibility([3, 1, 7], 6)
    False
    >>> subset_sum_divisibility([4, 6, 8, 8, 10, 10, 2, 3, 8, 14, 14, 3], 8)
    True
    """
    def subset_sum_memo(memo: dict, array: list, sum: int, m: int, index: int) -> bool:
        if sum and sum % m == 0:
            return True
        elif index >= len(array):
            return False
        else:
            value1, value2 = False, False
            if sum % m not in memo:
                value1 = subset_sum_memo(memo, array, sum + array[index], m, index + 1)
                value2 = subset_sum_memo(memo, array, sum, m, index + 1)
                if sum:
                    memo[sum % m] = value1 or value2
            if not sum:
                return value1 or value2
            return memo[sum % m]

    if len(array) >= m:
        return True
    memo = {}
    return subset_sum_memo(memo, array, 0, m, 0)
```

## Codechef problem: [Construct Array](https://www.codechef.com/LTIME80B/problems/CARR)

Given integer $N$ and $M$, construct an array $A$ in such a way, no three consecutive elements are the same, where $\forall\ a\in A, a\in[1,M]$.

When $n=1$, then answer is straightforward $M$, for $n=2$ the answer is $M^2$.

Generalizing this: we have for $f(n)$ two ways:
- We don't want three consecutive terms, so we'll fill the two array slots $A_{n}, A_{n-1}$ at the same time. This can be done in $M-1$ ways $\implies f(n-2)\cdot(M-1)$.
- We now have remaining value to fill $A_{n}$, in the previous point since we considered repeated values, we ignore them. So the results of this is $f(n-1)\cdot(M-1)$

$$
f(n)=\begin{cases}
M^n,&1\leq n\leq2,n\in\mathbb{N}\\
(M-1)\cdot(f(n-1)+f(n-2)),&n>2
\end{cases}
$$

This can be done efficiently by [[intermediate_maths_2#Linear Recurrence|matrix exponentiation]] in $O(2^3\log_2{n})$.

## Best Time to buy stocks.
- [ ] To do

# String problems

## Longest Substring Problem
One of the fastest algorithm $O(n)$ solution is a manacher's algorithm. But we're discussing other algorithm.
- [ ] To do

## Longest valid parantheses

Let string contains only $'('$ and $')'$. 
(e.g., $S=')()())'$), then longest valid parantheses is of length 4.

Note that we can search naively, and break when the brackets are unbalanced and $<0$, then we move starting position to the next.

```python
def longest_valid_parentheses(s):
	n, i, max_length = len(s), 0, 0
	# Till max_length, since there's no point in trying out 
	# next values
	while i < n - max_length:
		j, balance = i, 0
		while j < n and balance >= 0:
			if s[j] == '(': balance += 1
			else: balance -= 1
			if balance == 0:
				max_length = max(max_length, j - i + 1)
			j += 1
		i += 1
	return max_length
```

Memoization approach:
- Base: if $i=0$, return there are zero combinations
- $i>0, S[(i-1)\ldots i]='()'$, then combine last result and add $2$.
- $i > 0, S[(i-1)\ldots i]='))'$, then we look back at the previous results:
	- Using value computed at $f(i-1)=k$ (say), and from $(i-1)^{th}$ position, we move back $k$ steps. If $S[i-1-k]='('$, then we found the balancing bracket for $S[i]$: increment the  and add results previous to this position i.e., $S[i-k-2]$ as well to see if there are any results that are balanced.

$$
f(n)=\begin{cases}
0, & i \leq 0\\
f(n-2)+2, & S[(i-1)\ldots i]='()'\\
f(n-1)+f(n-f(n-1)-2)+2, & S[(i-1)\ldots i]='))'\wedge
S[n-f(n-1)-1]='('\\
0, & \text{otherwise}
\end{cases}
$$

The resultant of $f(n)$ can be stored in $memo_n$, so that it can be retrieved later.
These operations are symmetrical, i.e., these can be operated from $i=n\rightarrow0$, as well as $i=0\rightarrow n$. Base case and signs inside recursive variable function changes accordingly.

We'll have it stored in $memo[n]$, a 1-dimensional list/hashmap.

```python
def longest_valid_parantheses(string: str) -> int:
    """
    Determine substring containing balanced parantheses
    >>> longest_valid_parantheses('))(())')
    4
    >>> longest_valid_parantheses(')()())')
    4
    >>> longest_valid_parantheses(')()()(()))')
    8
    >>> longest_valid_parantheses(')()())()()(')
    4
    """
    def longest_naive(string: str) -> int:
        """
        Naive search, takes O(n^2)
        """
        n, i, max_length = len(string), 0, 0
        # Till max_length, since there's no point in trying out 
        # next values
        while i < n - max_length:
            j, balance = i, 0
            while j < n and balance >= 0:
                if string[j] == '(': balance += 1
                else: balance -= 1
                if balance == 0:
                    max_length = max(max_length, j - i + 1)
                j += 1
            i += 1
        return max_length

    def longest_valid_parentheses_recursive(memo: dict, string: str, position: int) -> int:
        """
        Searches longest balanced parantheses in string.
        """
        if position <= 0:
            return 0
        elif position not in memo:
            value = 0
            if string[position] == ')':
                if string[position-1] == '(':
                    value = 2 + longest_valid_parentheses_recursive(memo, string, position-2)
                else:
                    n_1 = longest_valid_parentheses_recursive(memo, string, position-1)
                    if position-n_1-1 >= 0 and string[position-1-n_1] == '(':
                        value = 2 + n_1 + longest_valid_parentheses_recursive(memo, string, position-n_1-2)
            memo[position] = value
        return memo[position]

    memo, max_value = {}, 0
    for x in range(len(string)-1, 0, -1):
        if x not in memo:
            longest_valid_parentheses_recursive(memo, string, x)
    for index, value in memo.items():
        max_value = max(max_value, value)
    return max_value
```
