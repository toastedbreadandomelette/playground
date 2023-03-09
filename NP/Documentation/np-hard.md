# NP - hard
NP - hard problem are a set that are 'at least as hard as hardest problem in [[np|NP]] set.'

Statement:
A problem $H$ is $NP$-hard (say $NPH$= set of NP-hard problems) when every problem $L$ in $NP$ set can be reduced in polynomial time to $H$.
i.e., assuming that solving $H$ takes 1 unit time, $H$'s solution can used to solve problem $L$ in polynomial time.

These problems may not be a decidable as well, and set $NP$-hard and $NP$ intersect $\text{NPH}\not\subset\text{NP}$. The intersection is a common set of decision problems that are reducible from $L$ to $H$.

If a certain problem is a **decision problem** that requires non-deterministic turing machine ($x\in \text{NP}$) and also are hardest problems, that are **reducible** from a problem $L$ to $H$ ($x\in \text{NPH}$), then the problem is said to be [[np-complete|NP-complete]] problem ($x \in \text{NPC}$).

The below results are concerned with problem set domain: 
$x\in(\text{NP }\cup\text{ NPH }\cup\text{ NPC})$:
- Problem $x\in \text{NPC} \implies x\in \text{NPH},\ \because \text{NPC}\subseteq\text{NPH}$.
i.e., For a problem $x$, which are NP-complete can also be called NP-hard problems as well. 

Halting problem, one of the problems being an exception (decision problem which can be reduced to a different problem), is an $\text{NP}$-hard problem since it is an undecidable problem.



