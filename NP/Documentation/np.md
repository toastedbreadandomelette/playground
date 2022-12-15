# NP Problem
NP is a complexity class, used to classify decision problems (a yes-no answer on certain input), these have solutions in polynomial time on a Non-deterministic Turing Machine (in generally $O(n^k)$ time, where $n$ is the input size).

All problems in $\text{P} \subseteq \text{NP}$ (reads as $\text{P}$ is a proper subset of $\text{NP}$).

# Problems
## Travelling Salesman Problem (Decision version)
Statement: Determine if there is a route that visits all vertex without crossing cost $x$.
Solution: Check all the routes, which takes $O(n!)$
Validation: 
- Using matrix table lookup $O(n)$
- Using adjacent lookup $O(n^2)$ or $O(n\log{n})$ if all vertex in adjacent matrix is in sorted order.


