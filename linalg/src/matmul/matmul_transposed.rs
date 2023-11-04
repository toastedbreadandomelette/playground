use crate::common::transpose_vec;

use vector::Vector;

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
