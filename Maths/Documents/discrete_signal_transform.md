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

```rust
pub mod complex {
    pub const PI: f64 = 3.141592653589793238462643383279;

    pub trait Number {}

    impl Number for u8 {}
    impl Number for u16 {}
    impl Number for u32 {}
    impl Number for usize {}
    impl Number for u64 {}
    impl Number for u128 {}
    impl Number for i8 {}
    impl Number for i16 {}
    impl Number for i32 {}
    impl Number for i64 {}
    impl Number for i128 {}
    impl Number for f32 {}
    impl Number for f64 {}
    // Complex number for computing.
    #[derive(Debug, Copy, Clone)]
    pub struct Complex {
        pub real: f64,
        pub img: f64,
    }
    // More impls
}

use std::ops::{AddAssign, Mul};
use complex::{Complex, PI};

pub fn dft<T: From<T> + AddAssign + Mul + Copy + complex::Number>(arr: &Vec<T>) -> Vec<Complex>
where
    f64: From<T>,
{
    let angle = PI * 2.0 / (arr.len() as f64);
    let wlen = Complex::new(angle.cos(), -angle.sin());
    let mut wstart = wlen;
    let mut result: Vec<Complex> = vec![Complex::new(0.0, 0.0); arr.len()];
    for x in arr {
        result[0] += *x;
    }

    for x in 1..arr.len() {
        let mut w = Complex::new(1.0, 0.0);
        for y in 0..arr.len() {
            result[x] += w * arr[y];
            w *= wstart;
        }
        wstart *= wlen;
    }

    result
}

pub fn idft<T: From<T> + AddAssign + Mul + Copy + complex::Number + std::convert::From<f64>>(
    arr: &Vec<Complex>,
) -> Vec<T>
where
    f64: Into<T>,
{
    let angle = PI * 2.0 / (arr.len() as f64);
    let wlen = Complex::new(angle.cos(), angle.sin());
    let mut wstart = wlen;
    let mut result: Vec<Complex> = vec![Complex::new(0.0, 0.0); arr.len()];
    for x in arr {
        result[0] += *x;
    }

    for x in 1..arr.len() {
        let mut w = Complex::new(1.0, 0.0);
        for y in 0..arr.len() {
            result[x] += w * arr[y];
            w *= wstart;
        }
        wstart *= wlen;
    }

    result
        .iter()
        .map(|x| (x.real / result.len() as f64).into())
        .collect::<Vec<T>>()
}
```
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

Combining two vectors: $A_{\text{odd}}$ and $A_{\text{even}}$.

$$
A'_x=A_{\text{odd}}
$$
- [ ] Todo: Proper explaination

```rust
pub fn idft_complex(arr: &Vec<Complex>) -> Vec<Complex> {
    let angle = PI * 2.0 / (arr.len() as f64);
    let wlen = Complex::new(angle.cos(), angle.sin());
    let mut wstart = wlen;
    let mut result: Vec<Complex> = vec![Complex::new(0.0, 0.0); arr.len()];
    for x in arr {
        result[0] += *x;
    }

    for x in 1..arr.len() {
        let mut w = Complex::new(1.0, 0.0);
        for y in 0..arr.len() {
            result[x] += w * arr[y];
            w *= wstart;
        }
        wstart *= wlen;
    }

    result
        .iter()
        .map(|x| *x / (result.len() as f64))
        .collect::<Vec<Complex>>()
}

pub fn fft<T: From<T> + AddAssign + Mul + Copy + complex::Number>(arr: &Vec<T>) -> Vec<Complex>
where
    f64: From<T>,
{
    if arr.len() < 16 || arr.len() & 1 == 1 {
        dft(arr)
    } else {
        let even = (0..arr.len())
            .step_by(2)
            .map(|x| arr[x])
            .collect::<Vec<T>>();
        let odd = (1..arr.len())
            .step_by(2)
            .map(|x| arr[x])
            .collect::<Vec<T>>();

        let odd_fft = fft(&odd);
        let even_fft = fft(&even);

        let angle = 2.0 * PI / (arr.len() as f64);
        let wlen = Complex::new(angle.cos(), -angle.sin());
        let mut w = wlen;
        let mut result: Vec<Complex> = Vec::with_capacity(arr.len());
        for x in 0..(arr.len() / 2) {
            result[x] = even_fft[x] + odd_fft[x] * w;
            result[x + arr.len() / 2] = even_fft[x] - odd_fft[x] * w;
            w *= wlen;
        }
        result
    }
}

pub fn ifft_internal(arr: &Vec<Complex>) -> Vec<Complex> {
    if arr.len() < 16 || arr.len() & 1 == 1 {
        idft_complex(arr)
    } else {
        let even = (0..arr.len())
            .step_by(2)
            .map(|x| arr[x])
            .collect::<Vec<Complex>>();
        let odd = (1..arr.len())
            .step_by(2)
            .map(|x| arr[x])
            .collect::<Vec<Complex>>();

        let odd_fft = ifft(&odd);
        let even_fft = ifft(&even);

        let angle = 2.0 * PI / (arr.len() as f64);
        let wlen = Complex::new(angle.cos(), -angle.sin());
        let mut w = wlen;
        let mut result: Vec<Complex> = Vec::with_capacity(arr.len());
        for x in 0..(arr.len() / 2) {
            result[x] = even_fft[x] + odd_fft[x] * w;
            result[x + arr.len() / 2] = even_fft[x] - odd_fft[x] * w;
            result[x] /= 2;
            result[x + arr.len() / 2] /= 2;

            w *= wlen;
        }
        result
    }
}

pub fn ifft(arr: &Vec<Complex>) -> Vec<f64> {
    ifft_internal(&arr)
        .iter()
        .map(|x| x.real.into())
        .collect::<Vec<f64>>()
}
```