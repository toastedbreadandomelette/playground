# Advanced Maths

## Partitioning Numbers
A paritition of a number is defined as a way of writing $n (\in\mathbb{N})$  as sum of integers (combinations).
For e.g., 
If $n=6$, then there are $11$ combinations:

$$\begin{array}{l}
6\\
5+1\\
4+2\\
4+1+1\\
3+3\\
3+2+1\\
3+1+1+1\\
2+2+2\\
2+2+1+1\\
2+1+1+1+1\\
1+1+1+1+1+1
\end{array}$$

The partition numbers $p(n)$ will be defined as possible partition of the number $n$: for e.g., $p(6)=11$.
A function $p(n)$ is defined:

$$
\begin{array}{cl}
p(n)=
\underbrace{p(n-1)+\rlap{\overbrace{\phantom{p(n-2)+p(n-5)}}^{\delta=3}}p(n-2)}_{\triangle=1}
-\underbrace{p(n-5)-\rlap{\overbrace{\phantom{p(n-7)+p(n-12)}}^{\delta=5}}p(n-7)}_{\triangle=2}
+\underbrace{p(n-12)+\rlap{\overbrace{\phantom{p(n-15)+p(n-12)}}^{\delta=7}}p(n-15)}_{\triangle=3}
-\underbrace{p(n-22)-p(n-26)}_{\triangle=4}+\cdots\\
\end{array}
$$

```python
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
```

## Linear Recurrence Relations - 2
[[linear_recurrence|Click here for Matrix Exponentiation]].

Source: https://discuss.codechef.com/t/linear-recurrence-using-cayley-hamilton-theorem/6776

## Solving Diophatine Equation (Pell's equation)


