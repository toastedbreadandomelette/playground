# Divide and Conquer
Divide and conquer problem deals with solving smaller parts of problem and combining them when these smaller problems are solved.

## [[searching#Binary Search|Binary Search Method]]
A sorted array/space is searched by splitting that space into two halves.

## Strassen's Method for [[matrix#Matrix multiplication.|Matrix Multiplication]]
In terms of computer algorithm, the strassen's method of matrix multiplication can be much faster than naive multiplication for large matrices.

Considering matrices $A$ and $B$:

$$
\begin{array}{cc}
A=\begin{bmatrix}
A_{11}&A_{12}\\
A_{21}&A_{22}
\end{bmatrix}&
B=\begin{bmatrix}
B_{11}&B_{12}\\
B_{21}&B_{22}
\end{bmatrix}&
C=\begin{bmatrix}
C_{11}&C_{12}\\
C_{21}&C_{22}
\end{bmatrix}&
\end{array}
$$

The resultant $C$ is evaluated by multiplying sub-matrices 8 times:

$$
C=\begin{bmatrix}
C_{11}&C_{12}\\
C_{21}&C_{22}
\end{bmatrix}=\begin{bmatrix}
A_{11} B_{11}+A_{12}B_{21}&A_{11}B_{12}+A_{12}B_{22}\\
A_{21} B_{11}+A_{22}B_{21}&A_{21}B_{12}+A_{22}B_{22}\\
\end{bmatrix}
$$

The strassen's method introduces $7$ new matrices:

$$
\begin{array}{l}
P_1=(A_{11}+A_{12})\cdot(B_{11}+B_{22})\\
P_2=(A_{21}+A_{22})\cdot B_{11}\\
P_3=A_{11}\cdot(B_{12}-B_{22})\\
P_4=A_{22}\cdot(B_{21}-B_{11})\\
P_5=(A_{11}+A_{12})\cdot(B_{22})\\
P_6=(A_{21}-A_{11})\cdot(B_{11}+B_{12})\\
P_7=(A_{21}-A_{22})\cdot(B_{21}+B_{22})
\end{array}
$$

With this, matrix $C$ is evaluated:

$$
\begin{array}{l}
C_{11}=P_1+P_4-P_5+P_7\\
C_{12}=P_3+P_5\\
C_{21}=P_2+P_4\\
C_{22}=P_1-P_2+P_3+P_6
\end{array}
$$

This is performed recursively till it can be divided no further/matrix becomes small enough to not be able to outperform the naive multiplication.

Complexity: since the recurrence relation is

$$
\begin{array}{c}
T(n)&=&7\cdot T\left(\dfrac{n}2\right)+k\\
&=&7^{\log_2{n}}\cdot T\left(\dfrac{n}{2^p}\right)\\
&=&n^{\log_27}\cdot T(1)\\
&\approx&n^{2.8073...}
\end{array}
$$

where $2^p\leq n<2^{p+1}$. We're considering $2^p=n$.

$$
\implies T(n)=O(n^{2.8073...})\text{ (strassen's method)} < O(n^3) \text{ (Naive multiplication)}
$$
