use crate::common::vector::Vector;

/// Multiply two matrices `a` and `b` of size
/// `ashape (m x n)` and `bshape(n x p)`
///
/// Returns new matrix vector `c` of size (m x p)
pub fn matmul_normal(
    a: &[f64],
    b: &[f64],
    ashape: (usize, usize),
    bshape: (usize, usize),
) -> Vector<f64> {
    assert!(ashape.1 == bshape.0);
    let (m, n, p) = (ashape.0, ashape.1, bshape.1);
    let mut c: Vector<f64> = Vector::zeroed(m * p);
    for i in 0..m {
        for j in 0..p {
            for k in 0..n {
                c[i * p + j] += a[i * n + k] * b[k * p + j];
            }
        }
    }
    c
}

/// Multiply two matrices `a` and `b` of size
/// `ashape (m x n)` and `bshape(n x p)`
///
/// Returns new matrix vector `c` of size (m x p)
pub fn matmul_normal_vec(
    a: &[f64],
    b: &[f64],
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