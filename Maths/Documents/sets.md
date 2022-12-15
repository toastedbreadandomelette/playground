# Sets
Sets are a collection of elements that might or might not follow a certain pattern.
Some of the examples:
$A=\{1,3,100,22222,10^{-9},e^{\pi}\}$ is a valid set of numbers, that does not follow any visible pattern.
$B=\{n\ |\ n\equiv 0\pmod{4}\}$: a set builder notation, denotes set of integers where $n$ divides $4$.

# Some of the well-defined sets:
$\mathbb{N}=\{1,2,3,\ldots\}$: set of natural numbers (we'll use $N$ sometimes)
$\mathbb{W}=\{0,1,2,3,\ldots\}$: set of whole numbers 
$\mathbb{Z}=\{\ldots,-3,-2,-1,0,1,2,3,\ldots\}$: set of integers 
$\mathbb{Q}=\left\{\dfrac{a}b,\quad a,b\in\mathbb{Z},\ b\neq0\right\}$: set of rational numbers
$\mathbb{R}$: Other numbers that are not rational: e.g., $\pi$, $e$, any $n^{th}$ root that is not perfect $n^{th}$ power and more, belong to this set.
$\mathbb{C}=\{x: x=a+bi, a,b\in\mathbb{R}\}$.

```ad-info
Note that $\mathbb{N}\subseteq\mathbb{W}\subseteq\mathbb{Z}\subseteq\mathbb{Q}\subseteq\mathbb{R}\subseteq\mathbb{C}$.
```
# Notations:
## Set builder
$B=\{n\ |\ n\in\mathbb{N},\ n\equiv 0\pmod{4}\}=\{4,8,12,16,20,\ldots\}$ is a generator/set builder notation.
# Functions
A function or a mapping from a set $A$ to set $B$ is a rule assigning each input from set $A$ to certain element in set $B$.
for e.g., 
$A=\{1,2,3,4,5,\ldots\}$, and $B=\{1,3,5,7,9,\ldots\}$
A function $f(x)=2x-1$ is a mapping from $A$ to $B$.
# Types:
## Bijective (One-to-One correspondence)
Bijective means there is only one pair between one element of set $A$ and one and only one element of set $B$. The above set is straight forward example of Bijection.
## Surjection
Surjection of two sets means that for all elements in set $B$, there exists at least one element mapped with set $A$.

Let $B=\{0,1,4,9,16,\ldots\}$ and $A=\mathbb{Z}$.
Then $f(x)=x^2$.
Then there are at most two values in set $\mathbb{Z}$ that are mapped to set $B$.
## Injection
- [ ] To do

# Cardinality
Cardinality is defined as total elements/members in the set. Cardinality of said set $A$ is denoted by $n(A)$ or $|A|$.
For e.g., for set $A=\{1,2,3,4\}$, $|A|=4$.
We've seen some infinite sets. Some of these sets are countable sets, and some sets that are uncountable.
Countable Sets: $\mathbb{N},\mathbb{W},\mathbb{Z}$.  
Uncountable Sets: $\mathbb{Q},\mathbb{R},\mathbb{C}$.
