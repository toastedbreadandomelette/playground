# Statistics

## Mean
Mean is the sum of all the numbers divided by total numbers (basically an [[basic_maths_2#Arithmetic mean of a number/series|Arithmetic mean]]).
 
$$\therefore AM(A)=\dfrac{\sum\limits_{i=1}^n a_i}{n}=\dfrac{a_1+a_2+a_3+\ldots+a_n}{n}$$

## Median
Median is a number or value in the dataset that, iff sorted in ascending order by it's value, that value divides the dataset in two sets.

i.e., in set $A,\ A_i\lt A_{i+1}\ \forall\  i\in[1,|A|],\ 1\leq i\leq |A|$, median $Me(A)$ is defined as:

$$\text{Med}(A)=\begin{cases}
A_{k},&k=\dfrac{|A|+1}2,\ |A|\equiv1\pmod2\\
\dfrac{A_{k_1}+A_{k_2}}2,&k_1=\dfrac{|A|}2,\ k_2=\dfrac{|A|+1}2,\ |A|\equiv0\pmod2
\end{cases}$$

# Mode
Mode is a observation value in dataset that occurs most of the time in the dataset. 

$$\text{Mode}(A)=\lbrace|C|, \forall c\in C,\ (c\in A)\wedge \left(c_i = c_j\ \forall\ i,\ j\in[0,|C|)\right)\rbrace$$

## Root Mean Square
Root mean square is square root of an average of all the values sqaured.

$$RMS(A)=\sqrt{\dfrac{1}n\sum_{i=0}^{|A|}a_i^2}$$

## [Standard Deviation](https://en.wikipedia.org/wiki/Standard_deviation)
Standard deviation is square root of average of all the values subtracted by mean of these values sqaured.

$$\sigma(A)=\sqrt{\dfrac{1}n\sum_{i=0}^{|A|}(\text{AM}(A)-a_i)^2}$$