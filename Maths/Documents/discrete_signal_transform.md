## [Fourier Transform](https://en.wikipedia.org/wiki/Fourier_transform)
Fourier Transform is a transform on a function that converts into another function defining frequencies present in fourier. It's output is a complex - valued function.

It's mathematical form:
$$
\begin{array}{}
f'(\xi)=\int_{-\infty}^{\infty} f(x)\cdot e^{-i2\pi\xi x} dx& \forall\  x\in R
\end{array}
$$
It's inverse is also evaluated as:
$$
\begin{array}{}
f(x)=\int_{-\infty}^{\infty} f'(\xi)\cdot e^{-i2\pi\xi x} d\xi &\forall\ x\in R
\end{array}
$$
## Discrete Fourier Transform (DFT)
Discrete Fourier Transform (DFT) is an operation on finite and equally separated values (or say function $f$) to complex - valued function.

This term is frequently used with Fast Fourier Transform (FFT).
Discrete Fourier Transform of set of points $X = \{x_0,x_1,\ldots,x_{n-1}\}$, i.e., $X'=\{x'_0,x'_1,\ldots,x'_{n-1}\}$.

$$
x'_k=\sum_{p=0}^{N-1}x_p\cdot \exp\left(-i\cdot2\pi\cdot k\cdot \dfrac{p}{N}\right)
$$
It's inverse (Inverse Discrete Fourier Transform or IDFT) is defined as:
$$
x_k=\dfrac{1}N\sum_{p=0}^{N-1}x'_p\cdot \exp\left(i\cdot2\pi\cdot k\cdot \dfrac{p}{N}\right)
$$
## Fast - Fourier Transform (FFT)
Fast - Fourier Transform (FFT) is a technique used to solve DFT operation (or IDFT).

**1**. **Divide**.
This is done by dividing values (considering it as polynomial of degree $N-1$), into odd and even coefficient for array of values $A=\{a_0,a_1,\ldots,a_{n-1}\}$.
i.e., 
For polynomial
$$
A(x)=a_0+a_1\cdot x+a_2\cdot x^2+\ldots+a_{n-1}\cdot x^{n-1}
$$
We recursively divide it (evenly) into two parts, until it cannot be divided evenly:
$$
\begin{matrix}
A_{\text{odd}}(x)=a_1+a_3\cdot x+a_5\cdot x^2+\ldots+a_{n-1}\cdot x^{n-1}\\
A_{\text{even}}(x)=a_0+a_2\cdot x+a_4\cdot x^2+\ldots+a_{n-2}\cdot x^{n-2}\\ \\
\implies A(x)=x\cdot A_{\text{odd}}(x^2)+A_{\text{even}}(x^2)
\end{matrix}
$$
Then we perform DFT (or FFT if it can be divided evenly again) on these separately. This is done till size of the array is 2.

**2**. **Combine**
To combine these values; consider an evaluated DFT of $p$ size (say). Combining these values in $A'$ array:

Combining two arrays: $A_{\text{odd}}$ and $A_{\text{even}}$.

$$
A'_x=A_{\text{odd}}
$$