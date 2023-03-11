# Matrix
Matrix is a rectangular array filled with numbers.

A typical matrix of $m$ rows and $n$ columns (or $m\times n$ matrix) would look like:

$$
A=\begin{bmatrix}
a_{11}&a_{12}&a_{13}&\cdots&a_{1n}\\
a_{21}&a_{22}&a_{23}&\cdots&a_{2n}\\
a_{31}&a_{32}&a_{33}&\cdots&a_{3n}\\
\vdots&\vdots&\vdots&\ddots&\vdots\\
a_{m1}&a_{m2}&a_{m3}&\cdots&a_{mn}\\
\end{bmatrix}
$$

# Types of matrices
## Row vector
These are $1\times n$ vector. (Below is a $1\times 5$ matrix)

$$\begin{bmatrix}1&10&8&9&6\end{bmatrix}$$
## Column vector
These are $n\times 1$ vector. (Below is a $5\times 1$ matrix)

$$\begin{bmatrix}
1\\
10\\
8\\
9\\
6
\end{bmatrix}$$

## Square matrices

These are $n\times n$ matrices: (Below is a $3\times 3$ matrix)

$$
\begin{bmatrix}
1&7&5\\
10&8&9\\
8&-1&6
\end{bmatrix}
$$

### Identity matrix $I$
These are also square matrices where

$$
A_{ij}=\left\{\begin{array}{cl}
1,&i=j\\
0,&i\neq j
\end{array}\right.
$$

i.e., a $4\times 4$ identity matrix would look like:

$$
I_4=\begin{bmatrix}
1&0&0&0\\
0&1&0&0\\
0&0&1&0\\
0&0&0&1\\
\end{bmatrix}
$$

### Symmetric matrix
Here, the values are symmetrical along the diagonals of a square matrix. i.e., $A_{ij}=A_{ji}$.

An example would look like this:

$$
S=\begin{bmatrix}
0&16&7&6\\
16&5&5&16\\
7&5&4&2\\
6&16&2&-1\\
\end{bmatrix}
$$

It's alternate definition is a matrix when transpose is the same as original matrix: i.e., $A^T=A$.

### Upper/lower/diagonal triangular matrix:
An upper triangular matrix is defined as matrix where $A_{ij}=0$ when $i > j$.

$$
A=\begin{bmatrix}
1&2&3&-1\\
0&7&6&0\\
0&0&12&3\\
0&0&0&1
\end{bmatrix}
$$

An lower triangular matrix is defined as matrix where $A_{ij}=0$ when $i < j$.

$$
A=\begin{bmatrix}
1&0&0&0\\
2&8&0&0\\
3&4&12&0\\
-1&3&9&1
\end{bmatrix}
$$

A diagonal matrix is defined as matrix where $A_{ij}=0$ when $i\neq j$.

$$
A=\begin{bmatrix}
1&0&0&0\\
0&8&0&0\\
0&0&12&0\\
0&0&0&1
\end{bmatrix}
$$

# Operations on matrices.

## Addition
Addition of two matrices $A$ and $B$ involves adding each of their respective $i,j$ cells ($A_{ij}+B_{ij}$). The requirement of matrices is the shape of both matrices should be same (say $m\times n$).

$$
\begin{matrix}
C&=&A+B\\\\
&=&\begin{bmatrix}
a_{11}&a_{12}&a_{13}&\ldots&a_{1n}\\
a_{21}&a_{22}&a_{23}&\ldots&a_{2n}\\
a_{31}&a_{32}&a_{33}&\ldots&a_{3n}\\
\vdots&\vdots&\vdots&\ddots&\vdots\\
a_{m1}&a_{m2}&a_{m3}&\ldots&a_{mn}
\end{bmatrix}+\begin{bmatrix}
b_{11}&b_{12}&b_{13}&\ldots&b_{1n}\\
b_{21}&b_{22}&b_{23}&\ldots&b_{2n}\\
b_{31}&b_{32}&b_{33}&\ldots&b_{3n}\\
\vdots&\vdots&\vdots&\ddots&\vdots\\
b_{m1}&b_{m2}&b_{m3}&\ldots&b_{mn}
\end{bmatrix}\\\\
&=&\begin{bmatrix}
a_{11}+b_{11}&a_{12}+b_{12}&a_{13}+b_{13}&\ldots&a_{1n}+b_{1n}\\
a_{21}+b_{21}&a_{22}+b_{22}&a_{23}+b_{23}&\ldots&a_{2n}+b_{2n}\\
a_{31}+b_{31}&a_{32}+b_{32}&a_{33}+b_{33}&\ldots&a_{3n}+b_{3n}\\
\vdots&\vdots&\vdots&\ddots&\vdots\\
a_{m1}+b_{m1}&a_{m2}+b_{m2}&a_{m3}+b_{m3}&\ldots&a_{mn}+b_{mn}\\
\end{bmatrix}
\end{matrix}
$$

## Scalar Multiplication.
Multiplying with a certain number (say constant $c$ with matrix $A$):

$$
\begin{matrix}
C&=&c\cdot A\\\\
&=&c\cdot\begin{bmatrix}
a_{11}&a_{12}&a_{13}&\ldots&a_{1n}\\
a_{21}&a_{22}&a_{23}&\ldots&a_{2n}\\
a_{31}&a_{32}&a_{33}&\ldots&a_{3n}\\
\vdots&\vdots&\vdots&\ddots&\vdots\\
a_{m1}&a_{m2}&a_{m3}&\ldots&a_{mn}
\end{bmatrix}\\\\
&=&\begin{bmatrix}
c\cdot a_{11}&c\cdot a_{12}&c\cdot a_{13}&\ldots&c\cdot b_{1n}\\
c\cdot a_{21}&c\cdot a_{22}&c\cdot a_{23}&\ldots&c\cdot b_{2n}\\
c\cdot a_{31}&c\cdot a_{32}&c\cdot a_{33}&\ldots&c\cdot b_{3n}\\
\vdots&\vdots&\vdots&\ddots&\vdots\\
c\cdot a_{m1}&c\cdot a_{m2}&c\cdot a_{m3}&\ldots&c\cdot b_{mn}\\
\end{bmatrix}\\\\
\end{matrix}
$$

## Transpose

A Transpose (denoted by $A^T$) of a matrix $A$ is a resultant matrix formed by converting rows into columns (or columns into rows).

$$
\begin{matrix}
A^T&=&\begin{bmatrix}
a_{11}&a_{12}&a_{13}&\ldots&a_{1n}\\
a_{21}&a_{22}&a_{23}&\ldots&a_{2n}\\
a_{31}&a_{32}&a_{33}&\ldots&a_{3n}\\
\vdots&\vdots&\vdots&\ddots&\vdots\\
a_{m1}&a_{m2}&a_{m3}&\ldots&a_{mn}
\end{bmatrix}^T\\\\
&=&\begin{bmatrix}
a_{11}&a_{21}&a_{31}&\ldots&a_{m1}\\
a_{12}&a_{22}&a_{32}&\ldots&a_{m2}\\
a_{13}&a_{23}&a_{33}&\ldots&a_{m3}\\
\vdots&\vdots&\vdots&\ddots&\vdots\\
a_{1n}&a_{2n}&a_{3n}&\ldots&a_{mn}
\end{bmatrix}\\\\
\end{matrix}
$$

The resultant matrix would be of shape $n\times m$.

## Matrix multiplication.
Resultant $C_{ij}$ from two matrices $A$ (of size $m\times n$) and $B$ (of size $n\times p$) (here total columns in $A$ should be equal to total rows in matrix $B$) is defined:

$$
\begin{array}{cl}
C_{ij}&=&\sum\limits_{k=1}^{n}A_{ik}\cdot B_{kj}\\
&=&A_{i1}\cdot B_{1j}+A_{i2}\cdot B_{2j}+\ldots+A_{in}\cdot B_{nj}
\end{array}
$$

We perform these operations for every pair of $i,j$ where $i\in[1,n]$ and $j\in[1,p]$. The overall result will be:

$$
\begin{matrix}
C&=&A\cdot B\\
&=&\begin{bmatrix}
a_{11}&a_{12}&a_{13}&\ldots&a_{1n}\\
a_{21}&a_{22}&a_{23}&\ldots&a_{2n}\\
a_{31}&a_{32}&a_{33}&\ldots&a_{3n}\\
\vdots&\vdots&\vdots&\ddots&\vdots\\
a_{m1}&a_{m2}&a_{m3}&\ldots&a_{mn}
\end{bmatrix}\cdot\begin{bmatrix}
b_{11}&b_{12}&b_{13}&\ldots&b_{1p}\\
b_{21}&b_{22}&b_{23}&\ldots&b_{2p}\\
b_{31}&b_{32}&b_{33}&\ldots&b_{3p}\\
\vdots&\vdots&\vdots&\ddots&\vdots\\
b_{n1}&b_{n2}&b_{n3}&\ldots&b_{np}
\end{bmatrix}\\\\
&=&\begin{bmatrix}
\sum\limits_{k=1}^na_{1k}\cdot b_{k1}&\sum\limits_{k=1}^na_{1k}\cdot b_{k2}&\sum\limits_{k=1}^na_{1k}\cdot b_{k3}&\ldots&\sum\limits_{k=1}^na_{1k}\cdot b_{kp}\\
\sum\limits_{k=1}^na_{2k}\cdot b_{k1}&\sum\limits_{k=1}^na_{2k}\cdot b_{k2}&\sum\limits_{k=1}^na_{2k}\cdot b_{k3}&\ldots&\sum\limits_{k=1}^na_{2k}\cdot b_{kp}\\
\sum\limits_{k=1}^na_{3k}\cdot b_{k1}&\sum\limits_{k=1}^na_{3k}\cdot b_{k2}&\sum\limits_{k=1}^na_{3k}\cdot b_{k3}&\ldots&\sum\limits_{k=1}^na_{3k}\cdot b_{kp}\\
\vdots&\vdots&\vdots&\ddots&\vdots\\
\sum\limits_{k=1}^na_{nk}\cdot b_{k1}&\sum\limits_{k=1}^na_{nk}\cdot b_{k2}&\sum\limits_{k=1}^na_{nk}\cdot b_{k3}&\ldots&\sum\limits_{k=1}^na_{nk}\cdot b_{kp}\\
\end{bmatrix}\\\\
\end{matrix}
$$

The resultant matrix $C$ will be of size $n\times p$.

## Determinant

Determinant of a square matrix $A$ (denoted by $\det(A)$) is defined as:

$$\det(A)=\sum\limits_{i=1}^{n}(-1)^{1+i}\cdot A_{1i}\cdot \det(M_{1,i})$$
We can also use any row as well for finding determinant as well. e.g., for any $k^{th}$ row, $\det(A)$ is:

$$\det(A)=\sum\limits_{i=1}^{n}(-1)^{k+i}\cdot A_{ki}\cdot \det(M_{k,i})$$
$\implies \det(A)=\sum\limits_{i=1}^{n}C_{k,i}\cdot A_{ki},\quad1\leq k\leq n$., where $C_{k,i}$ is [[matrix#Co-factor|cofactor]].
So, if

$$
A=\begin{bmatrix}
a_{11}&a_{12}&a_{13}&\ldots&a_{1n}\\
a_{21}&a_{22}&a_{23}&\ldots&a_{2n}\\
a_{31}&a_{32}&a_{33}&\ldots&a_{3n}\\
\vdots&\vdots&\vdots&\ddots&\vdots\\
a_{m1}&a_{m2}&a_{m3}&\ldots&a_{mn}
\end{bmatrix},
$$

then:

$$
\begin{array}{cl}
\det(A)=a_{11}\cdot\begin{vmatrix}
a_{22}&a_{23}&\ldots&a_{2n}\\
a_{32}&a_{33}&\ldots&a_{3n}\\
\vdots&\vdots&\ddots&\vdots\\
a_{m2}&a_{m3}&\ldots&a_{mn}
\end{vmatrix}-a_{12}\cdot\begin{vmatrix}
a_{21}&a_{23}&\ldots&a_{2n}\\
a_{31}&a_{33}&\ldots&a_{3n}\\
\vdots&\vdots&\ddots&\vdots\\
a_{m1}&a_{m3}&\ldots&a_{mn}
\end{vmatrix}+\cdots +\\(-1)^{1+n}\cdot a_{1n}\cdot\begin{vmatrix}
a_{21}&a_{22}&\ldots&a_{2(n-1)}\\
a_{31}&a_{32}&\ldots&a_{3(n-1)}\\
\vdots&\vdots&\ddots&\vdots\\
a_{m1}&a_{m2}&\ldots&a_{m(n-1)}
\end{vmatrix}
\end{array}
$$

These are then computed recursively.

For e.g., for $3\times 3$ matrix suppose:

$$
A=\begin{bmatrix}1&2&3\\
4&5&6\\ 
7&8&9\end{bmatrix}
$$

$$
\det(A)=1\cdot \begin{vmatrix}5&6\\
8&9\end{vmatrix}-
2\cdot
\begin{vmatrix}4&6\\
7&9\end{vmatrix}+
3\cdot\begin{vmatrix}4&5\\
7&8\end{vmatrix}
=1\cdot(45-48)-2\cdot(36-42)+3\cdot(32-35)
=0
$$

## Minor
A minor of the entry $A_{ij}$ in a matrix $A$ (denoted by $M_{i,j}$) is defined as a submatrix obtained by removing $i^{th}$ row and $j^{th}$ column from the matrix.

If $A=\begin{bmatrix}1&2&3\\4&5&6\\7&8&9\end{bmatrix}$, then minor of $A_{21}$ is  $M_{2,1}=\begin{bmatrix}2&3\\8&9\end{bmatrix}$.

## Co-factor

A co-factor $C_{i,j}$ is evaluated by multiplying $\det{(M_{i,j})}$ by $(-1)^{i+j}$.

$\implies C_{2,1}=(-1)^{2+1}\det(M_{2,1})=(-1)^{2+1}(9\cdot 2 - 3\cdot 8)=(-1)\cdot(-6)=6$.

## Row Operations

There are three operations we can perform:

### Switch rows:

Any rows can be swapped with a different rows to serve the purpose:

$$
\begin{matrix}R_1\\R_2\\R_3\end{matrix}\begin{bmatrix}
1&7&6\\2&3&10\\7&6&12
\end{bmatrix}\xrightarrow{R_1\leftrightarrow R_3}
\begin{bmatrix}
7&6&12\\2&3&10\\1&7&6
\end{bmatrix}
$$

### Scalar Multiplying a row

A row can be multiplied by a single scalar number:

$$
\begin{matrix}
R_1\\
R_2\\
R_3
\end{matrix}\begin{bmatrix}
1&7&6\\
2&3&10\\
7&6&12
\end{bmatrix}
\xrightarrow{R_2\rightarrow R_2\cdot 2}
\begin{matrix}
R_1\\
R_2\\
R_3
\end{matrix}
\begin{bmatrix}
1&7&6\\
4&6&20\\
7&6&12
\end{bmatrix}
$$
### Adding rows
Two rows can be added (with a scalar) and merged with the result

$$
\begin{matrix}
R_1\\
R_2\\
R_3\end{matrix}
\begin{bmatrix}
1&7&6\\
2&3&10\\
7&6&12
\end{bmatrix}
\xrightarrow{R_2\rightarrow R_2+1\cdot R_1}
\begin{matrix}
R_1\\
R_2\\
R_3
\end{matrix}
\begin{bmatrix}
1&7&6\\
3&10&16\\
7&6&12
\end{bmatrix}
$$

# Row echelon form
A matrix is row echelon form iff:
- All rows consisting of zeros are at the bottom.
- Leading value in row should be strictly to right of the leading value in previous row.
An example:

$$
\begin{bmatrix}
1&4&1&2&-3\\
0&0&1&4&2\\
0&0&0&1&-6\\
\end{bmatrix}
$$
## Reduced Row echelon form
A matrix is row echelon form iff:
- It is in row echelon form
- Leading entries in each row is $1$.
- Each column containing leading $1$ has zeroes in it.

e.g., 

$$
\begin{bmatrix}
1&4&0&0&-3\\
0&0&1&0&2\\
0&0&0&1&-6\\
\end{bmatrix}
$$