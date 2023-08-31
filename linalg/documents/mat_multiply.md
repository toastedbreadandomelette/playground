# Matrix Multiplication

This note is an extension to the [[matrix#Matrix multiplication.|original topic]]. 

Matrix multiplication of two matrix $A$ of size $m\times n$ and $B$ of size $n\times p$ is defined:

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

Matrix $C$ requires total columns of matrix $A$ to be same as rows of matrix $B$. 

At the moment, we're using [[arrays_1d|1-dimensional array]] to store a matrix. As a result, the array would be

$$
A = \begin{bmatrix}
A_{11}&A_{12}&\ldots&A_{1n}&|&
A_{21}&A_{22}&\ldots&A_{2n}&|&\ldots&|&\ldots&|&
A_{m1}&A_{m2}&\ldots&A_{mn}
\end{bmatrix}
$$

Noting that these $|$ symbol is shown as a compartment, but in fact stored in a `vec<f64>`.

```rust
/// Multiply two matrices `a` and `b` of size
/// `ashape (m x n)` and `bshape(n x p)`
/// 
/// Returns new matrix vector `c` of size (m x p)
pub fn matmul(
    a: &Vec<f64>,
    b: &Vec<f64>,
    ashape: (usize, usize),
    bshape: (usize, usize),
) -> Vec<f64> {
    assert!(ashape.1 == bshape.0);
    let (m, n, p) = (ashape.0, ashape.1, bshape.1);
    let mut c: Vec<f64> = vec![0.0; m * p];
    for i in 0..m {
        for j in 0..p {
            for k in 0..n {
                c[i * p + j] += a[i * n + k] * b[k * p + j];
            }
        }
    }
    c
}
```

## Optimization #1
Noting that matrix $B$ jumps $n$ steps results in too many cache misses. We can rearrange them such that fetching array values is sequential.
```rust
/// Multiply two matrices `a` and `b` of size
/// `ashape (m x n)` and `bshape(n x p)`
///
/// Returns new matrix vector `c` of size (m x p)
pub fn matmul_ikj(
    a: &Vec<f64>,
    b: &Vec<f64>,
    ashape: (usize, usize),
    bshape: (usize, usize),
) -> Vec<f64> {
    assert!(ashape.1 == bshape.0);
    let (m, n, p) = (ashape.0, ashape.1, bshape.1);
    let mut c: Vec<f64> = vec![0.0; m * p];
    for i in 0..m {
        for k in 0..n {
            for j in 0..p {
                c[i * p + j] += a[i * n + k] * b[k * p + j];
            }
        }
    }
    c
}
```

## Optimization #2
Another way to read values sequentially is to transpose the matrix. This converts our problem from matrix multiplication to dot product of each vector.

$$
\begin{matrix}
C_{ij}&=&A_i \cdot B^T_j,& \forall\ i\in [1,m], j\in [1,p]\\
&=&\sum\limits_{k=0}^{n} A_{ik}\cdot B^T_{jk}
\end{matrix}
$$

```rust
/// Compute transpose of matrix `A`
///
/// Also requires to pass shape of matrix, where
/// `m` is total rows in matrix, and `n` is total columns
/// in matrix
pub fn transpose_vec(
    a: &[f64],
    (m, n): (usize, usize),
) -> (Vector<f64>, (usize, usize)) {
    let mut ta: Vector<f64> = Vector::zeroed(a.len());
    // let rblock: usize = 32;

    a.chunks(n).enumerate().for_each(|(i, avec)| {
        avec.iter().zip(ta.iter_mut().skip(i).step_by(m)).for_each(
            |(aval, ta)| {
                *ta = *aval;
            },
        );
    });

    (ta, (n, m))
}

/// Multiply two matrices `a` and `b` of size
/// `ashape (m x n)` and `bshape(n x p)`
///
/// Difference is that the natrix `b` is transposed and
/// transposed matrix is used as a way to multiply
///
/// Returns new matrix vector `c` of size (m x p)
pub fn matmul_transposed(
    a: &[f64],
    b: &[f64],
    ashape: (usize, usize),
    bshape: (usize, usize),
) -> Vector<f64> {
    assert!(ashape.1 == bshape.0);
    let (m, n, p) = (ashape.0, ashape.1, bshape.1);
    let c: Vector<f64> = Vector::zeroed(m * p);

    // Before computing matrix multiplication:
    // we transpose them
    let (tb, _) = transpose_vec(b, (n, p));

    a.chunks(n).zip(c.chunks_mut(p)).for_each(|(avec, cvec)| {
        tb.chunks(n).zip(cvec).for_each(|(bvec, cval)| {
            *cval = avec
                .iter()
                .zip(bvec)
                .fold(0.0, |prev, (a1, b1)| prev + (a1 * b1));
        });
    });
    c
}
```

## Optimization #3
On a transposed matrix, we're directly storing the values in result $C_{ij}$. Instead we'll just use multiple accumulators.

```rust
/// Internal: Dot product of two vectors.
///
/// Returns the value
#[inline]
fn dot(avec: &[f64], bvec: &[f64]) -> f64 {
    avec.iter()
        .zip(bvec)
        .fold(0.0, |prev, (a1, b1)| prev + (a1 * b1))
}

/// Internal: Dot product of two vectors, but two are computed at a time.
///
/// Returns the pair (`avec . bvec0`, `avec . bvec1`)
#[inline]
fn dot2(avec: &[f64], bvec0: &[f64], bvec1: &[f64]) -> (f64, f64) {
    avec.iter()
        .zip(bvec0)
        .zip(bvec1)
        .fold((0.0, 0.0), |(p0, p1), ((a1, b1), b2)| {
            (p0 + (a1 * b1), p1 + a1 * b2)
        })
}

/// Internal: Dot product of two vectors, but three are computed at a time
///
/// Returns the tuple (`avec . bvec0`, `avec . bvec1`, `avec . bvec2`)
#[inline]
fn dot3(
    avec: &[f64],
    bvec0: &[f64],
    bvec1: &[f64],
    bvec2: &[f64],
) -> (f64, f64, f64) {
    avec.iter().zip(bvec0).zip(bvec1).zip(bvec2).fold(
        (0.0, 0.0, 0.0),
        |(p0, p1, p2), (((a1, b1), b2), b3)| {
            (p0 + (a1 * b1), p1 + a1 * b2, p2 + a1 * b3)
        },
    )
}

/// Internal: Dot product of two vectors, but four are computed at a time
///
/// Returns the tuple (`avec . bvec0`, `avec . bvec1`, `avec . bvec2`, `avec . bvec3`)
#[inline]
pub fn dot4(
    avec: &[f64],
    bvec0: &[f64],
    bvec1: &[f64],
    bvec2: &[f64],
    bvec3: &[f64],
) -> (f64, f64, f64, f64) {
    avec.iter()
        .zip(bvec0)
        .zip(bvec1)
        .zip(bvec2)
        .zip(bvec3)
        .fold(
            (0.0, 0.0, 0.0, 0.0),
            |(p0, p1, p2, p3), ((((a1, b1), b2), b3), b4)| {
                (p0 + (a1 * b1), p1 + a1 * b2, p2 + a1 * b3, p3 + a1 * b4)
            },
        )
}

/// Multiply two matrices `a` and `b` of size
/// `ashape (m x n)` and `bshape(n x p)`
///
/// Difference is that the natrix `b` is transposed and
/// transposed matrix is used as a way to multiply.
///
/// Also accumulation is done by computing 4 cells per iteration.
///
/// Returns new matrix vector `c` of size (m x p)
///
/// This can also be stated as `1x4` kernel multiplication
pub fn matmul_transposed_multi_accumulated(
    a: &[f64],
    b: &[f64],
    ashape: (usize, usize),
    bshape: (usize, usize),
) -> Vector<f64> {
    assert!(ashape.1 == bshape.0);
    let (m, n, p) = (ashape.0, ashape.1, bshape.1);
    let mut c: Vector<f64> = Vector::zeroed(m * p);

    // Before computing matrix multiplication:
    // we transpose them
    let (tb, _) = transpose_vec(b, (n, p));

    a.chunks(n).enumerate().for_each(|(i, avec)| {
        tb.chunks_exact(n * 4).enumerate().for_each(|(jl, bvec)| {
            let j = jl * 4;
            // 4 adjacent are computed simultaneously
            // so that values in row `avec` is used once for multiple
            // mul operations at once, reducing branches
            (
                c[i * n + j],
                c[i * n + j + 1],
                c[i * n + j + 2],
                c[i * n + j + 3],
            ) = dot4(
                avec,
                &bvec[0..n],
                &bvec[n..2 * n],
                &bvec[2 * n..3 * n],
                &bvec[3 * n..4 * n],
            );
        });
        // Residual operation is done after the above computation
        let val = tb.chunks_exact(n * 4).remainder();
        match val.len() / n {
            1 => c[i * n + n - 1] = dot(avec, val),
            2 => {
                (c[i * n + n - 2], c[i * n + n - 1]) =
                    dot2(avec, &val[0..n], &val[n..2 * n])
            }
            3 => {
                (c[i * n + n - 3], c[i * n + n - 2], c[i * n + n - 1]) =
                    dot3(avec, &val[0..n], &val[n..2 * n], &val[2 * n..3 * n])
            }
            _ => {}
        }
    });
    c
}
```
