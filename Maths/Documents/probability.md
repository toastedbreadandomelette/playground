# Probability
Probability is numerical value of a event/proposition that will turn out to be true. This value is between $[0,1]$, where $0$ denotes an event that'll not happen at all, whereas $1$ denotes certainty that the event will occur.

From a sample possible set of events: called sample space $\Omega$, an event is defined as a statement that covers some part (possibly none or all events) of the sample space.

For e.g., in a dice throw event, set $\Omega=\{1,2,3,4,5,6\}$, the probabilty that during dice throw, the value is greater than $4$ (say event $A$), the set $A=\{5,6\}$. And the probability, denoted by $P(A)$ is:
$$
P(A)=\dfrac{n(A)}{n(\Omega)}=\dfrac{|A|}{|\Omega|}=\dfrac{1}{3}
$$
A complement of an event is denoted as the event that will not occur: denoted by $A'$ (or $A^c,\neg A, \sim A, \bar{A}$): it's probability is $P(A')=1-P(A)$. In our example, the set $A'=\{1,2,3,4\}$, and $P(A')=1-P(A)=1-\dfrac13=\dfrac23$. (A complementary event).

# Types of events
## Independent events
These are events that has joint probability:
$$
P(A\cap B)=P(A)\cdot P(B).
$$
## Mutually exclusive event
The events $A$ and $B$ are mutually exclusive events that cannot happen simultaneously: i.e., $P(A\cap B)=0$.

For e.g., probability that a dice thrown is $6$ or $5$ are $\dfrac16+\dfrac16=\dfrac13$.

## Non-mutually exclusive event
The events $A$ and $B$ are mutually exclusive events that might happen simultaneously: i.e., $P(A\cap B)\neq0$.

for e.g., probability that a card has a heart or face is: $\dfrac{13}{52}+\dfrac{12}{52}-\dfrac{3}{52}=\dfrac{21}{52}$. ($P(A\cap B)$ denotes cards that have face and heart is $\dfrac{13}{52}\times \dfrac{12}{52}=\dfrac{3}{52}$, which also aligns with the total face card with heart since deck of cards has exactly $3$ cards J, Q, K of single type).

