# Fenwick Trees
Fenwick trees are a fast range query solving Data Structure that solves update and range queries in $O(\log_2{n})$ time. These are implemented by construction of bits, so it's also called binary-indexed tree.

We generally use [[arrays_1d|arrays]] to construct fenwick trees.
# $0$ based indexed Fenwick Trees
## Building $1$-Dimensional Fenwick Tree $F$.
Fenwick trees are also called binary-indexed trees: i.e., the fenwick trees uses bits of index to calculate/store the values. We'll be working on $0$-based indexing

The way this work is: for indices of the form $i=\ldots\underbrace{1111111}_{p \text{ digits}}$ (that are trailing with ones), the tree $F$ will compute and store cumulative function of all the previous $2^p$ values (from index $i-2^p+1$ to $i$ both inclusive) in index $i$ of tree $F$.

Starting with $i=4_{10}=100_2$ till $i\lt |A|$, the value of $A_i$ will be stored in these mentioned indices:

$$
\begin{array}{cl}
i=i\ |\ (i+1)=5_{10}=101_2\\
i=i\ |\ (i+1)=7_{10}=111_2\\
i=i\ |\ (i+1)=15_{10}=1111_2\\
\vdots& \text{indices of form } \ldots111 (|\log_2|A||\text{ times) will continue}
\end{array}
$$
Similarly for index $i=16=10000_2$, the next indices that'll contain the combined values including $A_{16}$ will be:

$$
\begin{array}{cl}
i=i\ |\ (i+1)=17_{10}=10001_2\\
i=i\ |\ (i+1)=19_{10}=10011_2\\
i=i\ |\ (i+1)=23_{10}=10111_2\\
i=i\ |\ (i+1)=31_{10}=11111_2\\
i=i\ |\ (i+1)=63_{10}=111111_2\\
\vdots
\end{array}
$$

The main build algorithm will be:

$$\forall\ i\in[0,|A|),\ F_i=f(A_{i-2^p+1},\ A_{i-2^p+2},\ \ldots, A_{i})$$

where $p$ is a total trailing $1$'s in binary representation of index $i$.

Time complexity of building this fenwick tree $F$ will take $O(n\cdot \log_2n)$ time. 

Below code takes a bit different step though, for every $i$, the subsequent indices tracked by $j$ (starting from $j=i$) are combined as $F_j:=f(F_j, A_i),\ j:=j\ |\ (j+1)$: But the above interpretation remains the same.

### Proof for complexity: 

Notice that for every index $i$, there are combined results of previous $2^p$ values (value of $p$ is shown above as count of trailing ones).
The pattern follows from $i=0\rightarrow15$ with count/size of each $F_i$ as: 

$$\begin{array}{cl}i_2\rightarrow&0&1&10&11&100&101&110&111&1000&1001&1010&1011&1100&1101&1110&1111\\
i_{10}\rightarrow&0&1&2&3&4&5&6&7&8&9&10&11&12&13&14&15&\ldots\\
|F_i|\rightarrow&1&2&1&4&1&2&1&8&1&2&1&4&1&2&1&16&\ldots\end{array}$$

For e.g., for $i=15$, $F_{16}$ holds results of previous $16$ values. 
For an array $A$, the total operations (say $P(|A|)$ performed will be:

$$
P(|A|)=|A|+
\underbrace{1\cdot\left\lfloor\dfrac{|A|}{2}\right\rfloor}_{\text{1 on i } \equiv 1\pmod{2}}+
\underbrace{2\cdot\left\lfloor\dfrac{|A|}{4}\right\rfloor}_{\text{2 on i } \equiv 3\pmod{4}}+
\underbrace{4\cdot\left\lfloor\dfrac{|A|}{8}\right\rfloor}_{\text{4 on i } \equiv 7\pmod{8}}+
\cdots+
\underbrace{2^{k-1}\cdot\left\lfloor\dfrac{|A|}{2^{k}}\right\rfloor}_{2^{k-1}\text{ on i } \equiv 2^k-1\pmod{2^k}}
$$

Notice that $\lceil\log_2|A|\rceil-1\leq k\leq\lceil\log_2|A|\rceil$. Removing all the floor operations, we can simplify them as:

$$\begin{array}{cl}
P(A)&\approx&|A|+\underbrace{\dfrac{|A|}2+\dfrac{|A|}2+\dfrac{|A|}2+\cdots+\dfrac{|A|}2}_{k=\log_2|A| \text{ times}}\\
&\approx&|A|+|A|\cdot\log_2|A|
\end{array}$$

If each operation of $f$ takes $O(c)$ time, then the overall complexity is $O(c\cdot |A|\cdot \log_2|A|)$. (we consider operations of $f$ constant $O(1), c=1$ since we normally use add/xor operations).

## Querying ranges $[l,r]$.

To compute the values in the given range $[l,r]$, we'll see how to combine first $n$ results.

We know that the index $i$ in tree $F_i$ contains results of $2^p$ previous values (from $(i-2^p+1)\rightarrow i$). If that's the case all we need is to jump from $i$ to $i-2^p$. We'll denote the result with $Q(i)$.
E.g., for $i=45_{10}=101101_2\implies Q(i)=f(A_0,A_1,\ldots,A_{45})$. The operations are performed and stored as result (say $R$, initialized with appropriate value) as:

$$
\begin{array}{ccc}
\text{op}&\text{index changes}&\text{remarks}\\
R:=f(R,\ F_{45}),&i:=(i\ \&\ (i+1))-1\ (=43=101011_2)&f(A_{44},A_{45})\text{ performed}\\
R:=f(R,\ F_{43}),&i:=(i\ \&\ (i+1))-1\ (=39=100111_2)&f(A_{40},\ldots ,A_{45})\text{ performed}\\
R:=f(R,\ F_{39}),&i:=(i\ \&\ (i+1))-1\ (=31=11111_2)&f(A_{32},\ldots ,A_{45})\text{ performed}\\
R:=f(R,\ F_{31}),&i:=(i\ \&\ (i+1))-1\ (=-1=\underbrace{1111\ldots1111_2}_{32 \text{ bits}})&f(A_{0},\ldots ,A_{45})\text{ performed}
\end{array}
$$

This takes $O(\log_2n)$ time for evaluating first $i$ values.

To evaluate the range $l\rightarrow r$, we just have to evaluate $f^{-1}(Q(r), Q(l-1))$.

### Proof for complexity

Looking at the pattern: the way we jump across indices, we can see that the worst case happens when $i$ has one and only one trailing zero (of the form $\underbrace{11111\ldots111}_{\text{all }1's}0$), and no zeroes in the binary representation elsewhere. This worst case forces to remove $1$'s from the last position one bit at a time. The size of such a binary number is $\log_2i$.

## Updating value at position $i$.
There are certain differences with updating **by** a certain value with a method and updating **to** a certain value: and the latter being dependent on reversing the value inserted before. As discussed in [[arrays_1d#Difference Array|Difference Arrays]], similar constraints are raised here (same with the [[fenwick_trees_1#Querying ranges l r|range queries]]). If inverse operator does not exists, then updated tree $F$ would not hold the correct values.

```cpp
#include <vector>
#include <cassert>
#include <functional>
#include <iostream>

/**
 * @brief Combine values for an item in index 
 * 
 * @tparam _T general array type
 * @param __f fenwick tree
 * @param index index at which current value is situated
 * @param __in input array
 * @param other other value
 * @param func combining function
 * @returns void
 */
template<typename _T>
void combine(std::vector<_T>&__f,
             size_t index,
             size_t size,
             const _T&other, 
             const std::function<_T(const _T&A, const _T&B)>&func) {
    for (; index < size; index = index | (index + 1)) {
        __f[index] = func(__f[index], other);
    }
}

/**
 * @brief Create a fen tree object
 * 
 * @tparam _T template parameter for input array
 * @param __in input array
 * @param func combining function
 * @param init initializing function
 * @return std::vector<_T> fenwick tree.
 */
template<typename _T>
std::vector<_T> create_fen_tree(const std::vector<_T>&__in, const std::function<_T(const _T&A, const _T&B)>&func, _T init) {
    _T result = init;
    std::vector<_T>fen_tree(__in.size(), init);
    for (size_t i = 0; i < __in.size(); ++i) {
        combine<_T>(fen_tree, i, __in.size(), __in[i], func);
    }
    return fen_tree;
}

/**
 * @brief Returns the combined value of first n value in array
 * 
 * @tparam _T template type for array input
 * @param __f fenwick tree
 * @param right rightmost index of a fenwick tree to query for
 * @param func combining function
 * @param init initializing value for result to start
 * @return _T the result value
 */
template<typename _T>
_T first_n_query(std::vector<_T>&__f, size_t right, const std::function<_T(const _T&A, const _T&B)>&func, _T init) {
    _T res = init;
    for (; right < __f.size(); right = (right & (right + 1)) - 1) {
        res = func(res, __f[right]);
    }
    return res;
}

/**
 * @brief Return range query on the range [l, r]
 * For this inverse operator is necessary, since r already considers values from [0, l]
 * as well. For this, we compute [0, l-1] and remove these values to get the answer in
 * range [l, r].
 * 
 * for e.g., a = {8, 2, 1, 10, 5, 19, 7, 8, 12, 256, 234, 102, 11, 3}
 * 
 * range_query(left=6, right=12, func=add(a, b), inv_func=sub(a, b))
 * 
 * will be first_n_query(12)=675, first_n_query(5)=45
 * 
 * => range_query=630
 * 
 * @tparam _T template parameter of input array
 * @param __f fenwick tree
 * @param left left index as starting point of value
 * @param right right index as ending point of value
 * @param func combining function
 * @param inv_func inverse function to remove calculated value
 * @param init 
 * @return _T 
 */
template<typename _T>
_T range_query(std::vector<_T>&__f,
               size_t left, size_t right,
               const std::function<_T(const _T&A, const _T&B)>&func,
               const std::function<_T(const _T&A, const _T&B)>&inv_func,
               _T init) {
    return inv_func(first_n_query(__f, right, func, init), first_n_query(__f, left - 1, func, init));
}

/**
 * @brief Update the value at a given position by changing the value by 
 * using func.
 * 
 * Note: Changing a certain element *to* a certain value is also possible:
 * FOR A SINGLE ELEMENT ONLY, only difference would be changing the value
 * 
 * @tparam _T type array
 * @param __f fenwick tree input
 * @param pos position for updating the value
 * @param func combining function for update
 * @param value value to update
 */
template<typename _T>
void single_update_by_using(std::vector<_T>&__f,
               std::vector<_T>&__in,
               size_t pos,
               const std::function<_T(const _T&A, const _T&B)>&func,
               _T value) {
    __in[pos] =func(__in[pos], value);
    for (; pos < __f.size(); pos = pos | (pos + 1)) {
        __f[pos] = func(__f[pos], value);
    }
}

/**
 * @brief Update the value at a given position to a certain value
 * 
 * @tparam _T type array
 * @param __f fenwick tree input
 * @param __in input of an array
 * @param pos position for updating the value
 * @param func combining function for update
 * @param inv_func inverse operator for removing values from fen_tree
 * @param value value to update
 */
template<typename _T>
void single_update_to(std::vector<_T>&__f,
               std::vector<_T>&__in,
               size_t pos,
               const std::function<_T(const _T&A, const _T&B)>&func,
               const std::function<_T(const _T&A, const _T&B)>&inv_func,
               _T value) {
    _T previous = __in[pos];
    __in[pos] = value;
    for (; pos < __f.size(); pos = pos | (pos + 1)) {
        __f[pos] = inv_func(__f[pos], previous);
        __f[pos] = func(__f[pos], value);
    }
}

int main () {
    auto func = [](const int &a, const int &b) {
        return a^b;
    };
    auto inv_func = [](const int &a, const int &b) {
        return a^b;
    };

    std::vector<int> mp = {8, 2, 1, 10, 5, 19, 7, 8, 12, 256, 234, 102, 11, 3};
    std::vector<int> fen = create_fen_tree<int>(mp, func, 0);

    int ans = range_query<int>(fen, 6, 12, func, inv_func, 0);
    std::cout << ans << std::endl;
    ans = first_n_query<int>(fen, 4, func, 0);
    std::cout << ans << std::endl;
    ans = range_query<int>(fen, 10, 12, func, inv_func, 0);
    std::cout << ans << std::endl;

    single_update_to<int>(fen, mp, 3, func, inv_func, 20);

    ans = first_n_query<int>(fen, 4, func, 0);
    std::cout << ans << std::endl;

    return 0;
}
```
